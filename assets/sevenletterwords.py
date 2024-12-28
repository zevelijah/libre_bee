# file_path
# Re-import necessary libraries and re-process the uploaded file.

input_path = 'assets/word-list-raw.txt'

# Read the provided word list and filter seven-letter words with unique letters
unique_seven_letter_words = []

try:
    # Process the file
    with open(input_path, 'r') as file:
        for word in file:
            word = word.strip().lower()  # Remove whitespace and convert to lowercase
            if len(word) == 7 and len(set(word)) == 7 and word.isalpha():  # Check conditions
                unique_seven_letter_words.append(word)

    # Write the filtered words to a new file
    output_path = 'assets/seven_unique_letter_words.txt'
    with open(output_path, 'w') as output_file:
        output_file.write('\n'.join(unique_seven_letter_words))

    output_path

except FileNotFoundError:
    output_path = "File not found. Please upload the file again."

print(f'file: {output_path}')