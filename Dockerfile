FROM rust

WORKDIR /app

COPY pre-build/ocr-rust /app/ocr-rust

CMD ["/app/ocr-rust"]