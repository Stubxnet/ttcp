TARGET = ttcp
SOURCE_DIR = .
BUILD_DIR = target/release
DESKTOP_FILE = sys/ttcp.desktop
MAN_PAGE = assets/ttcp.1
ICON_FILE = assets/ttcp.png
INSTALL_BIN_DIR = /usr/local/bin
INSTALL_DESKTOP_DIR = /usr/share/applications
INSTALL_MAN_DIR = /usr/share/man/man1
INSTALL_ICON_DIR = /usr/share/icons

build: clean
	cargo build --release -v
	cp $(BUILD_DIR)/$(TARGET) $(SOURCE_DIR)

install: build
	sudo cp $(SOURCE_DIR)/$(TARGET) $(INSTALL_BIN_DIR)/$(TARGET)
	sudo cp $(DESKTOP_FILE) $(INSTALL_DESKTOP_DIR)
	sudo cp $(ICON_FILE) $(INSTALL_ICON_DIR)
	sudo chmod +x $(INSTALL_DESKTOP_DIR)/ttcp.desktop
	sudo cp $(MAN_PAGE) $(INSTALL_MAN_DIR)
	sudo mandb

uninstall:
	sudo rm -f $(INSTALL_BIN_DIR)/$(TARGET)
	sudo rm -f $(INSTALL_DESKTOP_DIR)/ttcp.desktop
	sudo rm -f $(INSTALL_ICON_DIR)/ttcp.png

uninstall-doc:
	sudo rm -f $(INSTALL_MAN_DIR)/ttcp.1
	sudo mandb

clean:
	cargo clean

post-install:
	sudo chmod +x $(INSTALL_DESKTOP_DIR)/ttcp.desktop

.PHONY: build install install-doc uninstall uninstall-doc clean post-install
