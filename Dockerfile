
FROM jitesoft/tesseract-ocr:latest

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

CMD [ "cargo","run" ]