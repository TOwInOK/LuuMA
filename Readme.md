# Luuma
This bot can find some cool images from rule34 by your reqwest!

# Link
[Link to use my self hosted bot](https://discord.com/oauth2/authorize?client_id=1299512869649645634)

# Info

## providers
- Rule34
- Waifu.pics
- make issue for more...
## commands
- Rule34
  - /porno (only nsfw)
    - positive_tags (optional)
    - negative_tags (optional)
    - id (optional)
    - size (optional) (how many pics at once)
- Waifu.pics
  - /emotions (avaiable all emotion's types from waifu.pics)
  - /waifu (avaiable all waifu's types from waifu.pics)
  - /waifu_nsfw (same as waifu but nsfw)
  - /characters (avaiable all character's types from waifu.pics)
  - /action (avaiable all action's types from waifu.pics)
  - **note** all w.p commands has args:
    - many (if true = 4 pics, if false = 1 pic)
    - now (only in user bot mode, push message without choose pic) (you can delete your message in this mode)
## showcase
  ### default mode (if bot on server):
  <img width="596" alt="image" src="https://github.com/user-attachments/assets/d8c3851e-46ab-4b09-933e-6302565db01e" />
  
  ### default mode with many (if bot on server):
  <img width="448" alt="image" src="https://github.com/user-attachments/assets/862aa5b1-6d73-4c3c-acac-3552e0d51812" />

  ### now mode:
  <img width="550" alt="image" src="https://github.com/user-attachments/assets/bddf65bd-8bd8-455c-8813-e9f7f650d631" />
  <img width="383" alt="image" src="https://github.com/user-attachments/assets/3c89bb8b-dad3-49b0-a898-77a9f70e8dee" />


## Docker
```sh
docker run -d \
  -e DS_TOKEN=YOUR_TOKEN \
  ghcr.io/towinok/shuller_bot:latest
```

## Docker-compose
```yml
services:
  shuller_bot:
    image: ghcr.io/towinok/shuller_bot:latest
    environment:
      - DS_TOKEN=YOUR_TOKEN
```
