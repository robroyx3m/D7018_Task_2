# Guessing_Game 2e

A user has to guess what the secret random number is!
The random number is between 1 and 100.
The number of tries are recorded and shown on each iteration aswell as when a successful guess is made.

Run the file and execute by using the crate run command from the given project folder.

Will output the entire guessing history when the right guess has been made in a seemingly random order. This is probably an effect of how the HashMap that was used in this task has saved it's data on the memeory heap, it will collect the data from start of heap to end and thus will find different values at different places depending on where it was saved this time.

Error handling:
If something else than a number is entered an output will be made to say that this isn't a number, try again.

Game will end on either a correct answer or the usual ctrl+c in thr terminal.
