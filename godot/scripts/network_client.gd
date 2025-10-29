extends Node2D

# var client: GameClient

func _ready():
	print("ready or not here I come")
	$NetworkClient.connect_to_server("127.0.0.1", 8080)
	print("Listen here")
	$NetworkClient.start_listening()


# func _on_connected():
# 	print("Successfully connected to game server!")
# 	# Send initial message
# 	client.send_message("Hello from Godot!")

# func _on_connection_failed(error: String):
# 	print("Connection failed: ", error)

# func _on_message_received(message: String):
# 	print("Received from server: ", message)
# 	# Handle game state updates here

# func _exit_tree():
# 	if client:
# 		client.disconnect()
