extends Control

@export var marker_symbol: String
@export var show_timer: bool
@export var default_score: int
@export var decrease_signal: String
@export var increase_signal: String
@export var score_parameter: String

var marker : Label
var score : Label
var timer : Label

var value: int

func _init(p_marker_symbol = "x", p_show_timer = false, p_default_score = 0, p_score_parameter = "", p_decrease_signal = "", p_increase_signal = ""):
	marker_symbol = p_marker_symbol
	show_timer = p_show_timer
	default_score = p_default_score
	score_parameter = p_score_parameter
	decrease_signal = p_decrease_signal
	increase_signal = p_increase_signal	

func _ready() -> void:
	marker = get_node("MarginContainer/VBoxContainer/PanelContainer/HBoxContainer/Marker")
	timer = get_node("MarginContainer/VBoxContainer/Timer")
	score = get_node("MarginContainer/VBoxContainer/PanelContainer/HBoxContainer/Score")
		
	marker.text = marker_symbol
	timer.visible = show_timer
	value = GameState.get(score_parameter) if score_parameter else default_score
	score.text = str(value)
	
	if decrease_signal:
		Signals.connect(decrease_signal, _on_decrease_signal)
	if increase_signal:		
		Signals.connect(increase_signal, _on_increase_signal)

func _on_decrease_signal(cost):
	value -= cost
	score.text = str(value)
	
func _on_increase_signal(gift):
	value += gift
	score.text = str(value)
