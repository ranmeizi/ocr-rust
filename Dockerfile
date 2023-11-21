
FROM jitesoft/tesseract-ocr:latest

RUN apt-get update -y && apt-get install curl -y

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

CMD [ "cargo","run" ]