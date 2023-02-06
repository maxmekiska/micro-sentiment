# Micro-sentiment
A tiny rust micro-service implementation for sentiment analysis.


## Test
- run `cargo test`

## Run service
- run `cargo build`

## Endpoints

### Root endpoint `/`
```shell
http://127.0.0.1:8000/
```
### Sentiment endpoint `/sentiment`

```shell
curl -X POST -H "Content-Type: application/json" -d '{"text": "This is not good."}' http://127.0.0.1:8000/sentiment
```
