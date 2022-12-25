import rsScrabble

solver = rsScrabble.WordFinder("../pyScrabble/scrabbleWords.txt")

board_msg = ""
board_msg += "6__2___6___2__6"
board_msg += "_5___3___3___5_"
board_msg += "__5___2_2___5__"
board_msg += "2__5___2___5__2"
board_msg += "____5_____5____"
board_msg += "_3___3___3___3_"
board_msg += "__2___2_2___2__"
board_msg += "6__2___a___2__6"
board_msg += "__2___2r2___2__"
board_msg += "_3___3_be3___3_"
board_msg += "____5__R__5____"
board_msg += "2__5___e___5__2"
board_msg += "__5___2_2___5__"
board_msg += "_5___3___3___5_"
board_msg += "6__2___6___2__6"

print(solver.get_best_play("systeme", board_msg))
print(solver.get_best_play("system0", board_msg))
print(solver.get_best_play("", board_msg))