---
title: 'Token Optimization: Pruning Unused Skills and Custom Instructions'
publish_date: 2026-08-18
tags: ['ai']

---

![Unused skills and instructions are still loaded into every prompt, consuming tokens unnecessarily](/posts/2026/8/18/184b1af7-9b8d-4e03-9eca-c81a1e367e2d.png)

ถ้าเราใช้ AI coding agent อยู่นานพอ เราจะพบว่ามันมี "skill" และ "custom instruction" ของเราเองอยู่เป็นสิบ บางตัวเราสร้างไว้แล้วลืมไป บางตัวไม่ได้ใช้มาหลายเดือน แต่ทุกตัวถูกโหลดเข้าไปใน context ทุกครั้งที่เราพิมพ์ prompt ไปด้วยกันหมด

บล็อกนี้จะมาพูดถึงวิธีตัดแต่ง (trim) skill และ custom instruction ที่ไม่ได้ใช้ เพื่อประหยัด token ลดค่าใช้จ่าย และช่วยให้ agent โฟกัสกับงานจริงได้ดีขึ้น เพราะทุกบรรทัดที่เราไม่ได้ใช้ก็กิน token เหมือนกัน

---

## TL;DR

- ทุก request ของ agent โหลด skill list และ custom instruction ทุกตัวไปด้วยกัน แม้จะไม่ได้ใช้
- Skill ที่ไม่ได้ใช้แต่ description ยาว = กิน token ฟรีทุก prompt
- Custom instruction แบบ global โหลดทุกไฟล์ ควรเปลี่ยนเป็น scoped ด้วย `applyTo`
- วิธีวัด: ประมาณจำนวน token จากขนาดไฟล์ (rule of thumb ~4 ตัวอักษรต่อ 1 token)
- วิธีตัด: ประเมินว่าใช้จริงหรือไม่ → ตัด description ที่ยาวเกินไป → scope ลง → ลบสิ่งซ้ำ
- เป้าหมายคือ "น้อยแต่ตรงเป้า" ไม่ใช่ "เยอะแต่ครบ"

---

## Token ของเราไปไหนทุกครั้งที่พิมพ์ prompt

หลายคนเข้าใจว่า token ที่ใช้ไปคือแค่สิ่งที่เราพิมพ์บวกคำตอบของ AI แต่จริง ๆ แล้วทุก request มีส่วนคงที่ที่ถูกส่งไปทุกทีด้วย

```
Every request to an AI agent includes:
  System prompt            (fixed)
  Skill list (name+desc)   (one entry per skill)
  Custom instructions      (loaded per applyTo)
  Chat history             (grows over time)
  ─────────────────────────────────
  YOUR ACTUAL PROMPT
```

สองบรรทัดที่เราควบคุมได้คือ skill list และ custom instructions ยิ่งมี skill เยอะ description ยาว หรือ instruction แบบ global มากเท่าไร ยิ่งมี "background noise" ที่ agent ต้องอ่านทุกครั้งแม้จะไม่มีส่วนกับงานตรงหน้าเลย

---

## ทำไม skill และ instruction ที่ไม่ได้ใช้ถึงเป็นปัญหา

### Skill

Skill มักถูกเก็บเป็น `SKILL.md` ที่มี YAML front matter บอก `name` และ `description` ระบบจะเอาแค่ name + description ขึ้นไปใส่ใน prompt เสมอ เพื่อให้ agent ตัดสินใจว่าจะเรียกใช้ skill ตัวไหน

```yaml
---
name: go-deploy
description: Build, test, and deploy Go services to Kubernetes.
  USE FOR: deploy, rollout, rollback, helm, k8s, prod release.
---
```

ปัญหาคือถ้าเราเคยสร้าง skill ไว้ใช้เฉพาะกิจแล้วลืมไป เช่น `go-deploy` ที่ไม่ได้แตะมาสามเดือน มันยังคงกิน token ทุก prompt ทั้งที่แทบไม่ได้ถูกเรียกเลย description ยิ่งยาว ยิ่งแพงโดยเปล่าประโยชน์

### Custom instruction

Custom instruction เช่น `copilot-instructions.md` หรือ `AGENTS.md` ถูกโหลดเข้าไปใน context แบบ global ถ้าเราเขียนยาวเป็นหน้า ๆ และใช้กับทุกไฟล์ ทั้งที่จริง ๆ มันควรใช้เฉพาะในบางโฟลเดอร์ มันก็จะถูกส่งไปทุก request เหมือนกัน

---

## วัดก่อนตัด: ประมาณ token footprint

ก่อนจะตัดอะไรต้องรู้ก่อนว่าอะไรหนักอะไรเบา วิธีง่ายสุดคือประมาณจากขนาดไฟล์ เพราะนับ token แม่นยำ 100% ยาก แต่ rule of thumb คือประมาณ 4 ตัวอักษรต่อ 1 token (สำหรับข้อความผสมไทย-อังกฤษ)

```bash
#!/usr/bin/env bash
# audit-skills.sh - list every skill and its description length so you
# can spot the ones that are rarely used but always loaded.
#
# Heuristic: 1 token ~= 4 characters for mixed Thai/English text.

set -euo pipefail

SKILLS_DIR=".agents/skills"

printf "%-30s %10s %10s\n" "SKILL" "DESC_BYTES" "EST_TOKENS"
printf "%-30s %10s %10s\n" "-----" "----------" "----------"

total=0
for skill in "$SKILLS_DIR"/*/SKILL.md; do
  name="$(basename "$(dirname "$skill")")"
  # The description lives in the YAML front matter.
  desc="$(sed -n 's/^description: //p' "$skill")"
  bytes="${#desc}"
  tokens=$(( bytes / 4 ))
  total=$(( total + tokens ))
  printf "%-30s %10d %10d\n" "$name" "$bytes" "$tokens"
done

echo "----------------------------------------"
echo "Total skill description tokens loaded every prompt: ~$total"
```

รันสคริปต์นี้แล้วเราจะเห็นเรียงจากใหญ่ไปเล็ก สิ่งที่เราทำคือหา skill ที่ description ใหญ่แต่ใช้หายาก แล้วเริ่มจากตรงนั้นก่อน

---

## 1. ตัด description ของ skill ให้กระชับ

หลักการคือ description ต้องสั้นพอที่ agent ตัดสินใจเร็วได้ แต่เฉพาะเจาะจงพอไม่สับสนกับ skill ตัวอื่น จุดที่ตัดได้บ่อยคือ list ของ keyword ที่ซ้ำกับชื่อ skill เอง

### ก่อนตัด

```yaml
---
name: microsoft-foundry
description: Deploy, evaluate, and manage Foundry agents end-to-end.
  USE FOR: deploy agent to Foundry, hosted agent, create agent,
  invoke agent, evaluate agent, run batch eval, optimize prompt,
  improve prompt, prompt optimization, prompt optimizer,
  dataset curation from traces, RBAC, role assignment,
  permissions, quota, capacity, region, troubleshoot agent,
  deployment failure, create dataset, knowledge index.
---
```

### หลังตัด

```yaml
---
name: microsoft-foundry
description: Deploy and evaluate Foundry agents.
  USE FOR: deploy, batch eval, optimize prompt, RBAC, troubleshoot.
---
```

keyword เดิม 25 ตัว เหลือ 6 ตัวแต่ยังครอบคลุม intent หลัก ได้ประโยชน์สองทาง ทั้ง token น้อยลง และ agent ไม่ต้องเสียสมาธิกับคำที่ซ้ำ

---

## 2. เปลี่ยน custom instruction แบบ global เป็น scoped

ถ้า instruction ใช้เฉพาะกับบางภาษาหรือบางโฟลเดอร์ อย่าทำให้มัน global ใช้ `applyTo` จำกัดขอบเขตแทน

### ก่อนตัด (global, โหลดทุกไฟล์)

```markdown
# Rust conventions

- Use `thiserror` for library error types, not `std::error::Error`.
- Prefer `?` propagation over manual error handling.
- Run `cargo clippy -- -D warnings` before committing.
```

### หลังตัด (scoped เฉพาะไฟล์ Rust)

```markdown
---
applyTo: "src/**/*.{rs}"
---

# Rust conventions

- Use `thiserror` for library error types, not `std::error::Error`.
- Prefer `?` propagation over manual error handling.
- Run `cargo clippy -- -D warnings` before committing.
```

ผลลัพธ์คือเวลาเราทำงานกับไฟล์ที่ไม่ใช่ Rust instruction นี้จะไม่ถูกโหลดเลย ประหยัด token ได้ทันทีโดยไม่เสียอะไร

---

## 3. ลบ skill และ instruction ที่ไม่ใช้จริง

การตัด description หรือ scoped ยังไม่พอ ถ้า skill ตัวนั้นไม่ได้ถูกเรียกใช้เลย สิ่งที่ดีที่สุดคือลบไปเลย วิธีประเมินคือดูว่าเราใช้มันครั้งสุดท้ายเมื่อไหร่ หรือลองสังเกตพฤติกรรมของ agent ว่าเป็นเพราะ skill ตัวนั้นทำให้ตอบเพี้ยนหรือเปล่า

ถ้าหาไม่ได้ชัดเจน ให้ใช้หลัก "ถ้า description ของมันไม่ช่วย agent ตัดสินใจเลือกงานที่เรากำลังทำ มันก็แค่ noise ให้ลบได้"

---

## สรุป

การตัดแต่ง skill และ custom instruction เป็นงานที่ "ดูไม่สำคัญแต่เห็นผลชัดเจน" เพราะทุกบรรทัดที่เราไม่ได้ใช้ก็ถูกส่งไปทุก prompt เหมือนกัน เราสามารถ optimization ได้ทันทีโดยไม่ต้องพึ่งเครื่องมือแพงหรือ model ใหญ่

- ทุก request โหลด skill list และ instruction ไปด้วยกัน แม้ไม่ได้ใช้
- Skill ที่ description ยาวแต่วันจริงใช้น้อย = กิน token ฟรีทุก prompt
- วัดก่อนตัดด้วย heuristic ~4 ตัวอักษรต่อ 1 token
- ตัด description ให้กระชับ เปลี่ยน global เป็น scoped ด้วย `applyTo` และลบสิ่งที่ไม่ใช้จริง
- เป้าหมายคือ context ที่ "น้อยแต่ตรงเป้า" ไม่ใช่ "เยอะแต่ครบ"

ท้ายที่สุดแล้ว context ที่สั้นและตรงจุดไม่แค่ประหยัดเงิน แต่ยังช่วยให้ agent โฟกัสกับงานได้แม่นยำขึ้น เพราะมันไม่ต้องเสียสมาธิไปกับข้อมูลที่ไม่ได้เกี่ยวข้องเลย