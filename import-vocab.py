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
    if len(sys.argv) != 3:
        print("Usage: ./import-vocab.py <language> <filename>")
        sys.exit()

    language = sys.argv[1]
    filename = sys.argv[2]
    conn = setup_db()

    with conn.cursor() as cursor:
        with open(filename, 'r', encoding="utf-8") as file:
            for lines in batched(file, 1000):
                params_list = []
                for line in lines:
                    [word] = line.split()
                    params_list.append([word, language])
                cursor.executemany("INSERT INTO vocab (word, lang) VALUES (%s, %s) ON CONFLICT DO NOTHING", params_list)
        
    conn.commit()

if __name__ == "__main__":
    main()
