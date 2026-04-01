extends Control

@export var marker_symbol: String
@export var show_timer: bool
@export var default_score: int

var marker : Label
var score : Label
var timer : Label

func _init(p_marker_symbol = "x", p_show_timer = false, p_default_score = 0):
	marker_symbol = p_marker_symbol
	show_timer = p_show_timer
	default_score = p_default_score

func _ready() -> void:
	marker = get_node("MarginContainer/VBoxContainer/PanelContainer/HBoxContainer/Marker")
	timer = get_node("MarginContainer/VBoxContainer/Timer")
	score = get_node("MarginContainer/VBoxContainer/PanelContainer/HBoxContainer/Score")
	
	marker.text = marker_symbol
	timer.visible = show_timer
	score.text = str(default_score)
