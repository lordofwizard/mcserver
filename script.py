from pynput.keyboard import Key, Controller

import time

keyboard = Controller()
while True:
	time.sleep(10)
	keyboard.type("neofetch")
	keyboard.press(Key.enter)
	keyboard.release(Key.enter)
