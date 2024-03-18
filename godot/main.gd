extends Node

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	$GameScene/HUD.update_score($GameScene.score)
