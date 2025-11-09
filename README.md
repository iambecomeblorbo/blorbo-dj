# ❄️ blorbo_dj 🎧

HELLO.  
i am blorbo. i am penguin.  

this my musik machine. it make THE VIBES.

---

## 📼 what is this

blorbo_dj is terminal-based musical contraption.  
press the button. blorbo play the sound.

you can:
- press `1` for LOFI ICE CAPS 🌨️  
- press `2` for TECHNO GROTTO 🕶️  
- press `3` for BLORBO THEME 🐟  
- press `4` for your own `.blorb` song (you must make it first)  
- press `d` to compile a `.blorb` file into mp3 (sound alchemy)  
- press `q` to cry and exit the terminal like a chilly coward

---

## 🧊 what is `.blorb`

`.blorb` is tiny musik language for penguin fingers.  

blorbo understands:
```

C4:0.5
E4:1.0
rest:0.25

```

each line is note + duration in seconds.  
use `rest` to stop yelling for a bit.  
put `.blorb` files in `music/scripts/`.

---

## 🧪 how to run the thing

```
cargo run
```

blorbo will open his icy little terminal club.  
type things. it make sound.

you must have:
- Rust (cargo)
- `ffmpeg` for sound converting
- brain frost protection not required but recommended

---

## 📀 example: compile your `.blorb`

in the blorbo DJ terminal, press:

```

d

```

you will be asked:

```

🎹 Enter filename from 'music/scripts/' (e.g. icebeat.blorb):

```

blorbo types:

```

icebeat.blorb

```

and then blorbo waits.  
and then... **music gets made**.

you can now press:

```

4

```

to play the `.blorb` song you just compiled.

the MP3 file will be at:

```

music/output/icebeat.mp3

```

✨ it is real. the vibes are real. the beat is cold.

---

## 🗂️ folder system:

```

music/
├── output/        → where .mp3s and .wavs go
├── scripts/       → where .blorb files live
├── static/        → preloaded lofi & techno tracks

```

put your custom `.blorb` files into `music/scripts/` like `icebeat.blorb`.

---

## ⚙️ tech stack (blorbo say these words, doesn't know what mean)

- Rust 🦀
- `fundsp` for bleep bloop
- `rodio` for playings
- `crossterm` for keyboard smashings
- `hound` for WAV thing
- `ffmpeg` to transmogrify wave to mp3

---

## 🐧 final thought

blorbo vibe coded his way into this project in **one Starbucks sitting**.  
no shame in that.  
blorbo is a penguin.

please enjoy responsibly.  
and do not let blorbo near subwoofer. he panic.
