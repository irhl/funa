CC = rustc
SRC = funa.rs
BIN = /bin/funa

install:
	$(CC) $(SRC) -o $(BIN)

uninstall:
	$(RM) $(BIN)
