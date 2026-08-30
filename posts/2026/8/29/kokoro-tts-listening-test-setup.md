---
title: Set up English listening practice with Kokoro TTS in Python
publish_date: 2026-08-29
tags: ['python', 'tts', 'kokoro', 'ai', 'side-project']
---

ช่วงนี้ผมกำลังช่วยลูกฝึกภาษาอังกฤษด้วยการทำแบบฝึกหัดฟัง มีคำศัพท์ 21 คำที่ต้องจำให้ได้เพื่อสอบเขียนในชั้นเรียน

เงื่อนไขการสอบคือคุณครูจะบอกคำศัพท์ทีละคำ แล้วให้เด็ก ๆ เขียนตาม ด้วยสำเนียงของผมเอง คงออกเสียงได้ไม่เหมือนคุณครูที่เป็นเจ้าของภาษาแน่ ๆ

เลยลองหาตัวเลือกเป็นโมเดล TTS ที่มีขนาดเล็กพอและให้เสียงสำเนียงอังกฤษได้ ก็เลยมาเจอกับ [Kokoro-82M](https://huggingface.co/hexgrad/Kokoro-82M) ซึ่งเป็นโมเดลขนาด 82 ล้านพารามิเตอร์ ใช้ได้ฟรี (Apache-2.0) และรันแบบ offline บนเครื่องได้เลย ผมเลยลองเอามาทำเป็นโปรแกรมออกเสียงเพื่อให้ลูกได้ใช้ทดสอบการฟัง

## TL;DR

[GitHub](https://github.com/nomkhonwaan/nomkhonwaan/blob/main/python/kokoro/main.py)

## Kokoro คืออะไร

Kokoro เป็นโมเดล text-to-speech ที่เล็กพอจะรันบน CPU หรือ Apple Silicon ได้ ติดตั้งคู่กับ `espeak-ng` สำหรับแปลงข้อความเป็นเสียง (grapheme-to-phoneme)

## จัดการคำศัพท์

หลักการคือฟังก์ชันสร้างเสียงแล้วส่ง WAV กลับในรูปแบบบิตสตรีมผ่าน HTTP:

```python
from io import BytesIO
import soundfile as sf
from kokoro import KPipeline

pipeline = KPipeline(lang_code="b", repo_id="hexgrad/Kokoro-82M")

def synthesize_wav(word: str) -> bytes:
    buf = BytesIO()
    for _gs, _ps, audio in pipeline(word, voice="bm_lewis", speed=1.0):
        sf.write(buf, audio.detach().cpu().numpy(), 24000, format="WAV", subtype="PCM_16")
    return buf.getvalue()
```

ฟังก์ชันนี้ output เป็น `bytes` ของไฟล์ WAV ในหน่วยความจำ เมื่อเบราว์เซอร์ส่ง `GET /audio?word=garden` เซิร์ฟเวอร์ก็เรียกฟังก์ชันนี้ แล้วตอบกลับเป็น `audio/wav` ให้เบราว์เซอร์เล่นได้ทันที มี `voice="bm_lewis"` เป็นเสียงผู้ชายสำเนียง British English

### โค้ดเต็มทำงานยังไง

แยกอธิบายเป็นส่วน ๆ เพื่อให้เห็นภาพรวมชัดขึ้น โปรแกรมแบ่งเป็นสี่ส่วนหลัก

#### 1. โหลดโมเดลไว้ล่วงหน้า

```python
pipeline = KPipeline(lang_code="b", repo_id="hexgrad/Kokoro-82M")
```

บรรทัดเดียวนี้คือจุดเริ่มต้นของทุกอย่าง ตรง `repo_id` บอกให้ดาวน์โหลดโมเดล Kokoro จาก Hugging Face ครั้งแรก แล้วเก็บ (cache) ไว้ในเครื่อง ส่วน `lang_code="b"` คือย่อมาจาก British English เพื่อกำหนดสำเนียง ขั้นนี้เป็นส่วนที่ช้าที่สุดของโปรแกรม เพราะต้องโหลดโมเดลทั้งหมดเข้าหน่วยความจำ แต่พอโหลดเสร็จหนึ่งครั้งแล้ว ตัวแปร `pipeline` ถูก reuse ไปตลอด เราไม่ต้องโหลดซ้ำทุกครั้งที่พูดคำใหม่

#### 2. สังเคราะห์เสียงในหน่วยความจำ

```python
def synthesize_wav(word: str) -> bytes:
    buf = BytesIO()
    for _gs, _ps, audio in pipeline(word, voice="bm_lewis", speed=1.0):
        sf.write(buf, audio.detach().cpu().numpy(), 24000, format="WAV", subtype="PCM_16")
    return buf.getvalue()
```

นี่คือฟังก์ชันหลัก รับคำหนึ่งคำ (`word`) เข้ามา แล้วให้ `pipeline` พูดคำนั้นออกมาเป็นข้อมูลเสียง ตรง `voice="bm_lewis"` เป็นเสียงผู้ชายสำเนียง British English

ผมเขียนผลลัพธ์ลง `BytesIO` ซึ่งเป็นบัฟเฟอร์ในหน่วยความจำ แล้ว `sf.write` ก็จัดรูปแบบข้อมูลเป็น WAV (PCM 16-bit ที่ความถี่ 24kHz) ก่อนส่งกลับ ฟังก์ชันจึงคืนค่าเป็น `bytes` ของไฟล์ WAV

#### 3. รับคำขอจาก HTTP

```python
# GET /                  -> returns the HTML page
# GET /audio?word=garden -> synthesizes "garden" and responds with audio/wav
```

เซิร์ฟเวอร์เปิดรับที่พอร์ต `8765` มี endpoint สองตัว:

- `GET /` - ตอบกลับด้วยหน้าเว็บ HTML ที่เราสร้างไว้
- `GET /audio?word=...` - เมื่อเบราว์เซอร์ร้องขอคำไหน จะเรียก `synthesize_wav()` แล้วตอบกลับเป็น `audio/wav` พร้อมบอก `Content-Length` ทำให้เบราว์เซอร์รู้ขนาดไฟล์ก่อนจะเล่น

#### 4. หน้าเว็บเล่นเสียง

ด้านหน้าเป็น HTML ที่ฝังไว้ในโค้ด มีแค่ปุ่ม 21 ปุ่ม แต่ละปุ่มแทนหนึ่งคำ พอกดปุ่ม JavaScript จะตั้ง `src` ของ `<audio>` ไปที่ `GET /audio?word=คำศัพท์` แล้วสั่งเล่นทันที

การออกแบบแบบนี้ทำให้เบราว์เซอร์ขอเสียงเฉพาะตอนที่คลิกจริง ๆ ผลลัพธ์คือเปิดเว็บครั้งแรกเร็วมาก เพราะตอนเปิดยังไม่ต้องสังเคราะห์เสียงคำใดเลย

โค้ดเต็มที่รวมทั้งหมดนี้อยู่ที่ `python/kokoro/main.py` ใน repository นี้ ลองเปิดดูได้เลย แต่ถ้าอยากลองรันเอง:

```bash
brew install espeak-ng            # mac: required for correct pronunciation
pip install kokoro torch soundfile
python python/kokoro/main.py
```

แล้วเปิด `http://127.0.0.1:8765` ในเบราว์เซอร์ คลิกคำไหนก็ได้ยินเสียงคำนั้นทันที โมเดลจะโหลดครั้งแรกตอนเปิดเว็บ แล้วค่อย ๆ โหลดถัดไปเรื่อย ๆ ครับ