---
title: Builder Pattern ใน Go
publish_date: 2026-08-12
tags: ['go']
---

Builder Pattern เป็นรูปแบบที่ใช้งานเมื่อการสร้างอ๊อบเจ๊คมีพารามิเตอร์หลายตัวหรือมีขั้นตอนซับซ้อน การแยกการตั้งค่าออกเป็นเมทอดแบบ chain จะช่วยให้โค้ดอ่านง่าย ลดโค้ดซ้ำ และจัดการค่าของ `struct` ได้เป็นระบบ ใน Go แนวทางนี้ทำให้โค้ดการสร้างตัวแปรที่ซับซ้อนดูสวยงามและสะอาดขึ้น

ตัวอย่างนี้จะใช้ไลบรารี `github.com/ibm-messaging/mq-golang/v5/ibmmq` เพื่อสาธิตการสร้างการเชื่อมต่อและการประกาศข้อความด้วย Builder

---

สมมติว่าเราต้องการเชื่อมต่อกับ Queue Manager โดยมีค่าที่ต้องกำหนดอย่าง `QueueManager`, `ConnectionName`, `Channel`, `UserID` และ `Password` หากเขียนโค้ดตรง ๆ เราต้องเตรียม `MQCNO`, `MQCD` และ `MQCSP` ด้วยตัวเอง ซึ่งทำให้โค้ดยาวและอ่านยาก

```go
package main

import (
    "fmt"
    "log"

    ibmmq "github.com/ibm-messaging/mq-golang/v5/ibmmq"
)

func buildConnectionDirectly() (*ibmmq.MQQueueManager, error) {
    cno := ibmmq.NewMQCNO()
    cno.Options = ibmmq.MQCNO_CLIENT_BINDING

    cd := ibmmq.NewMQCD()
    cd.ChannelName = "DEV.APP.SVRCONN"
    cd.ConnectionName = "localhost(1414)"
    cno.ClientConn = cd

    csp := ibmmq.NewMQCSP()
    csp.UserId = "app"
    csp.Password = "password"
    cno.SecurityParms = csp

    qm, err := ibmmq.Connx("QM1", cno)
    if err != nil {
        return nil, err
    }
    return qm, nil
}

func main() {
    qm, err := buildConnectionDirectly()
    if err != nil {
        log.Fatalf("failed to connect to MQ: %v", err)
    }
    defer qm.Disc()

    fmt.Println("connected to MQ queue manager", qm.Name)
}
```

โค้ดตัวอย่างนี้ทำงานได้ดี แต่รายละเอียดการสร้าง connection ถูกกระจายอยู่หลายจุด เมื่อเราต้องการเพิ่มตัวเลือก เช่น `SSLConfig` หรือ `Timeout` โค้ดก็จะยิ่งเดินหน้าลำบาก

---

Builder Pattern จะย้ายรายละเอียดการประกอบค่าไปไว้ใน Builder แล้วเปิด API ให้ฝั่งเรียกใช้งานอ่านง่ายขึ้น

```go
package main

import (
    "fmt"
    "log"

    ibmmq "github.com/ibm-messaging/mq-golang/v5/ibmmq"
)

type MQConnectionBuilder struct {
    qMgrName      string
    connectionName string
    channelName   string
    userID        string
    password      string
}

func NewMQConnectionBuilder() *MQConnectionBuilder {
    return &MQConnectionBuilder{}
}

func (b *MQConnectionBuilder) WithQueueManager(name string) *MQConnectionBuilder {
    b.qMgrName = name
    return b
}

func (b *MQConnectionBuilder) WithConnectionName(conn string) *MQConnectionBuilder {
    b.connectionName = conn
    return b
}

func (b *MQConnectionBuilder) WithChannel(channel string) *MQConnectionBuilder {
    b.channelName = channel
    return b
}

func (b *MQConnectionBuilder) WithCredentials(userID, password string) *MQConnectionBuilder {
    b.userID = userID
    b.password = password
    return b
}

func (b *MQConnectionBuilder) Build() (*ibmmq.MQQueueManager, error) {
    cno := ibmmq.NewMQCNO()
    cno.Options = ibmmq.MQCNO_CLIENT_BINDING

    cd := ibmmq.NewMQCD()
    cd.ChannelName = b.channelName
    cd.ConnectionName = b.connectionName
    cno.ClientConn = cd

    if b.userID != "" || b.password != "" {
        csp := ibmmq.NewMQCSP()
        csp.UserId = b.userID
        csp.Password = b.password
        cno.SecurityParms = csp
    }

    return ibmmq.Connx(b.qMgrName, cno)
}

func main() {
    builder := NewMQConnectionBuilder().
        WithQueueManager("QM1").
        WithConnectionName("localhost(1414)").
        WithChannel("DEV.APP.SVRCONN").
        WithCredentials("app", "password")

    qm, err := builder.Build()
    if err != nil {
        log.Fatalf("failed to connect to MQ: %v", err)
    }
    defer qm.Disc()

    fmt.Println("connected to MQ queue manager", qm.Name)
}
```

ข้อดีของ Builder คือโค้ดฝั่งใช้งานย่อมอ่านง่ายขึ้น และรายละเอียดการสร้าง connection ถูกซ่อนอยู่ใน Builder ทำให้เราสามารถเพิ่มตัวเลือกใหม่ได้โดยไม่ต้องแก้โค้ดใน `main`

---

อีกงานที่ Builder Pattern เหมาะมากคือการประกอบข้อความที่จะส่งใน IBM MQ เรามักต้องกำหนดค่าใน `MQMD` และ `MQPMO` หากใช้ Builder โค้ดฝั่งส่งจะดูชัดเจนและโฟกัสที่ค่าที่เปลี่ยนแปลงได้

```go
package main

import (
    "github.com/ibm-messaging/mq-golang/v5/ibmmq"
)

type MQPutMessageBuilder struct {
    md      *ibmmq.MQMD
    pmo     *ibmmq.MQPMO
    message []byte
}

func NewMQPutMessageBuilder() *MQPutMessageBuilder {
    return &MQPutMessageBuilder{
        md:  ibmmq.NewMQMD(),
        pmo: ibmmq.NewMQPMO(),
    }
}

func (b *MQPutMessageBuilder) WithFormat(format string) *MQPutMessageBuilder {
    b.md.Format = format
    return b
}

func (b *MQPutMessageBuilder) WithPersistence(persistence int32) *MQPutMessageBuilder {
    b.md.Persistence = persistence
    return b
}

func (b *MQPutMessageBuilder) WithReplyToQueue(replyTo string) *MQPutMessageBuilder {
    copy(b.md.ReplyToQName[:], []byte(replyTo))
    return b
}

func (b *MQPutMessageBuilder) WithMessage(body []byte) *MQPutMessageBuilder {
    b.message = body
    return b
}

func (b *MQPutMessageBuilder) Build() (*ibmmq.MQMD, *ibmmq.MQPMO, []byte) {
    return b.md, b.pmo, b.message
}
```

เมื่อใช้งาน Builder ฝั่งส่งข้อความก็จะเหมือนเป็นการประกาศค่าที่อ่านได้ง่าย ไม่ต้องกลับไปจัดการโครงสร้าง `MQMD` / `MQPMO` ซ้ำ ๆ

```go
func sendMessage(qm *ibmmq.MQQueueManager, queueName string) error {
    qObject, err := qm.Open(ibmmq.NewMQOD().SetObjectName(queueName), ibmmq.MQOO_OUTPUT)
    if err != nil {
        return err
    }
    defer qObject.Close(0)

    md, pmo, body := NewMQPutMessageBuilder().
        WithFormat("MQSTR").
        WithPersistence(ibmmq.MQPER_PERSISTENT).
        WithReplyToQueue("REPLY.QUEUE").
        WithMessage([]byte("hello from builder pattern")).
        Build()

    if err := qObject.Put(md, pmo, body); err != nil {
        return err
    }
    return nil
}
```

Builder Pattern เหมาะกับงานที่ต้องสร้างตัวแปรหลายตัวและต้องจัดการกับ API ที่ซับซ้อนอย่าง IBM MQ เพราะมันช่วยแยกส่วนการตั้งค่าจากส่วนที่เรียกใช้งานจริง ทำให้โค้ดฝั่งใช้งานดูสะอาดขึ้น โดยไม่ต้องรู้รายละเอียดภายในของ `MQCNO`, `MQCD`, `MQMD` หรือ `MQPMO`

---

Unit testing

Builder Pattern ทำให้การเขียนเทสต์ง่ายขึ้น เพราะเราไม่จำเป็นต้องพึ่งพา MQ server จริงในเทสต์ ตัวอย่างนี้เป็น unit test ของ `MQPutMessageBuilder` เพื่อยืนยันว่า `Build()` คืนค่า `MQMD` และ payload ตามที่ตั้งไว้

```go
package main

import (
    "testing"
    "github.com/ibm-messaging/mq-golang/v5/ibmmq"
)

func TestMQPutMessageBuilder_Build(t *testing.T) {
    b := NewMQPutMessageBuilder().
        WithFormat("MQSTR").
        WithPersistence(ibmmq.MQPER_NOT_PERSISTENT).
        WithReplyToQueue("REPLY.Q").
        WithMessage([]byte("payload"))

    md, _, body := b.Build()

    if md.Format != "MQSTR" {
        t.Fatalf("expected format MQSTR, got %s", md.Format)
    }
    if string(body) != "payload" {
        t.Fatalf("expected body 'payload', got %s", string(body))
    }

    reply := string(md.ReplyToQName[:])
    if len(reply) == 0 || reply[:7] != "REPLY.Q" {
        t.Fatalf("reply-to queue not set correctly: %q", reply)
    }
}
```

---

สรุปแล้ว Builder Pattern เป็นเครื่องมือที่ช่วยให้การสร้างอ๊อบเจ๊คที่มีพารามิเตอร์หลายตัวหรือมีขั้นตอนซับซ้อนดูเป็นระเบียบมากขึ้น ใน Go เราสามารถใช้เมทอดแบบ chain เพื่อแยกการตั้งค่าออกจากการทำงานจริง ทำให้โค้ดอ่านง่าย ลดโค้ดซ้ำ และเพิ่มความยืดหยุ่นในการปรับเปลี่ยนหรือเพิ่มตัวเลือกใหม่ในอนาคต

