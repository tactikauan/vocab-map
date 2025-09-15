#!/usr/bin/env python

import sys
import os
import psycopg
from dotenv import load_dotenv
from itertools import batched

def setup_db():
    load_dotenv()
    return psycopg.connect(os.getenv('DATABASE_URL'))

def main():
    if len(sys.argv) != 2:
        print("Usage: ./import.py <filename>")
        sys.exit()

    filename = sys.argv[1]
    conn = setup_db()

    with conn.cursor() as cursor:
        with open(filename, 'r', encoding="utf-8") as file:
            for lines in batched(file, 1000):
                params_list = []
                for line in lines:
                    values = line.split()
                    word = values[0]
                    vector = list(map(float, values[1:]))
                    params_list.append([word, vector])
                cursor.executemany("INSERT INTO embeddings (word, vector) VALUES (%s, %s)", params_list)
        
    conn.commit()

if __name__ == "__main__":
    main()
