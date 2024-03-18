extends CanvasLayer

# Notifies `Main` node that the button has been pressed
signal start_game

signal in_main_menu

signal game_over_signal

func _ready():
	game_over_signal.connect(show_game_over)
	game_over_signal.connect(stop_music)

func show_message(text):
	$Message.text = text
	$Message.show()
	$MessageTimer.start()
	
func stop_music():
	$Music.stop()
	$DeathSound.play()

func show_game_over():
	show_message("Game Over")
	# Wait until the MessageTimer has counted down.
	await $MessageTimer.timeout

	$Message.text = "Dodge the Creeps!"
	$Message.show()
	# Make a one-shot timer and wait for it to finish.
	await get_tree().create_timer(1.0).timeout
	$StartButton.show()
	in_main_menu.emit()

func update_score(score):
	$ScoreLabel.text = str(score)

func _on_start_button_pressed():
	$StartButton.hide()
	show_message("Get Ready!")
	$Music.play()
	start_game.emit()

func _on_message_timer_timeout():
	$Message.hide()
