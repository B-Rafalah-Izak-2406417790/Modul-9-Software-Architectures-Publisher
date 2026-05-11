# Publisher

## Reflection

### How much data will the publisher send to the message broker in one run?

In one run, the publisher sends five event messages to the message broker. Each message is a `UserCreatedEventMessage` published to the `user_created` queue with a different `user_id` and `user_name`.

### What does `amqp://guest:guest@localhost:5672` mean?

The URL tells the publisher to connect to the same RabbitMQ message broker used by the subscriber through the AMQP protocol. The first `guest` is the RabbitMQ username, the second `guest` is the password, and `localhost:5672` means the broker is running on the local machine at port `5672`. Since the publisher and subscriber use the same URL and queue name, they communicate through the same message broker.

## Running RabbitMQ as Message Broker

RabbitMQ was run using Docker with the management plugin enabled:

```powershell
docker run -it --rm --name rabbitmq -p 5672:5672 -p 15672:15672 rabbitmq:3.13-management
```

The AMQP broker listens on port `5672`, while the RabbitMQ Management UI is available at `http://localhost:15672`. The default username and password are both `guest`.

![RabbitMQ Management UI](Image/img.png)
