docker run \
    -it \
    --rm \
    -v ./compilador/:/home/compilador \
    -v ./corretor/:/home/corretor \
    -e PATH="corretor/:$PATH" \
    compilador-rust