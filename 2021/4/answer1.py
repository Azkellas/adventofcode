import sys

# lines = sys.stdin

lines = sys.stdin

boards = []
board = None
first_line = True
for line in lines:
    if first_line:
        nbs = [int(l) for l in line.split(',')]
        first_line = False
    elif line.strip() == "":
        if board is not None:
            print(board)
            boards.append(board)
        board = [[0 for i in range(5)] for j in range(5)]
        idx = 0
    else:
        board[idx] = [int(l) for l in line.split()]
        idx = idx + 1
# boards.append(board)

print(len(boards))
print(nbs)
# for board in board:
#     print(board)
#     for j in range(5):
#         for i in range(5):
#             print(board[i][j], end=" ")
#         print()
#     print()

def test_winner(board):
    for i in range(5):
        winner = True
        for j in range(5):
            if board[i][j] != -1:
                winner = False
                break
        if winner:
            return True
    for j in range(5):
        winner = True
        for i in range(5):
            if board[i][j] != -1:
                winner = False
                break
        if winner:
            return True
    return False

def count_points(board):
    sum = 0
    for i in range(5):
        for j in range(5):
            if board[i][j] != -1:
                sum = sum + board[i][j]
    return sum

def remove_number(board, nb):
    for i in range(5):
        for j in range(5):
            if board[i][j] == nb:
                board[i][j] = -1

for number in nbs:
    for board in boards:
        remove_number(board, number)
        if test_winner(board):
            scoreeboard = count_points(board)
            print(number, scoreeboard)
            print(number * scoreeboard)
            exit(0)

