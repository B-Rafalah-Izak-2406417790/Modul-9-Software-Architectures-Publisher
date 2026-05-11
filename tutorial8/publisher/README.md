# Publisher

## Refleksi

### Berapa banyak data yang dikirim publisher ke message broker dalam satu kali jalan?

Dalam satu kali jalan, program publisher mengirim lima event message ke message broker. Setiap message menggunakan struktur `UserCreatedEventMessage` dan dikirim ke routing key `user_created` dengan `user_id` dan `user_name` yang berbeda.

### Apa arti URL `amqp://guest:guest@localhost:5672`?

URL tersebut menunjukkan bahwa publisher terhubung ke message broker RabbitMQ melalui protokol AMQP. `guest` pertama adalah username RabbitMQ, `guest` kedua adalah password, sedangkan `localhost:5672` berarti broker berjalan di komputer lokal pada port `5672`. Karena publisher dan subscriber memakai alamat broker dan routing key yang sama, keduanya dapat berkomunikasi melalui RabbitMQ.

## Menjalankan RabbitMQ sebagai Message Broker

RabbitMQ dijalankan menggunakan Docker dengan management plugin:

```powershell
docker run -it --rm --name rabbitmq -p 5672:5672 -p 15672:15672 rabbitmq:3.13-management
```

Port `5672` digunakan oleh AMQP broker, sedangkan RabbitMQ Management UI dapat dibuka melalui `http://localhost:15672`. Username dan password default-nya adalah `guest`.

![RabbitMQ Management UI](Image/img.png)

## Mengirim dan Memproses Event

Setelah RabbitMQ berjalan, subscriber dijalankan terlebih dahulu dengan `cargo run`. Setelah itu publisher dijalankan dengan `cargo run` dari direktori publisher. Publisher mengirim lima event `UserCreatedEventMessage` ke RabbitMQ, lalu subscriber menerima dan memproses kelima event tersebut.

Pada screenshot berikut terlihat subscriber menerima lima message dengan `user_id` berbeda dari publisher.

![Subscriber menerima event dari publisher](Image/img_1.png)
