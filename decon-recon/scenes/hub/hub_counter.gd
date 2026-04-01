extends Control

@export var marker_symbol: String
@export var show_timer: bool

var marker : Label
var timer : Label

func _init(p_marker_symbol = "x", p_show_timer = false):
	marker_symbol = p_marker_symbol
	show_timer = p_show_timer

func _ready() -> void:
	marker = get_node("MarginContainer/VBoxContainer/PanelContainer/HBoxContainer/Marker")
	timer = get_node("MarginContainer/VBoxContainer/Timer")
	
	marker.text = marker_symbol
	timer.visible = show_timer
