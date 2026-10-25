#!/usr/bin/env python3
"""Seed Negatiview with demo reviewers, reviews, comments, follows and favorites.

Talks to the public HTTP API only, so it works against any running instance:

    python3 scripts/seed.py http://localhost:8081/api

Re-running is safe: existing users are logged in instead of re-created, and
their reviews are added again with fresh ids.
"""
import json
import sys
import urllib.error
import urllib.request

BASE = sys.argv[1].rstrip("/") if len(sys.argv) > 1 else "http://localhost:8081/api"
PASSWORD = "demo-pass-123"


def call(method, path, body=None, token=None):
    data = json.dumps({"data": body}).encode() if body is not None else None
    req = urllib.request.Request(BASE + path, data=data, method=method)
    req.add_header("Content-Type", "application/json")
    if token:
        req.add_header("Authorization", f"Bearer {token}")
    try:
        with urllib.request.urlopen(req) as resp:
            raw = resp.read()
            return json.loads(raw) if raw else {}
    except urllib.error.HTTPError as err:
        return {"error": err.code, "body": err.read().decode()[:200]}


USERS = [
    ("mina", "mina@example.com", "Audio nerd. I return more gadgets than I keep."),
    ("jace", "jace@example.com", "Buys the thing so you don't have to."),
    ("kai", "kai@example.com", "Kitchen appliances, tested to destruction."),
]

POSTS = {
    "mina": [
        (
            "AirBuds Pro 3: the ANC is a rumor",
            "Great marketing, mediocre noise cancelling, and a case that scratches if you look at it.",
            "## What they promise\n\n*Studio-grade* active noise cancelling.\n\n"
            "## What you get\n\n- Low rumble on a train? Mostly gone.\n"
            "- Office chatter? **Completely untouched.**\n"
            "- The case picked up scratches in my pocket within a week.\n\n"
            "## Verdict\n\nIf you already own the previous generation, keep them. "
            "The upgrade is a firmware update you can't download.",
            ["earbuds", "audio", "anc"],
        ),
        (
            "Volt 65W GaN charger throttles after ten minutes",
            "Hits 65 W for exactly long enough to pass a review, then settles at 38 W.",
            "I logged the output with a USB-C power meter for an hour:\n\n"
            "| Minute | Watts |\n|---|---|\n| 0–10 | 64 |\n| 10–20 | 51 |\n| 20+ | 38 |\n\n"
            "The brick gets too hot to hold. A charger that only charges fast while it is cold "
            "is a 38 W charger.",
            ["charger", "usb-c"],
        ),
    ],
    "jace": [
        (
            "The Nimbus Air 14 is a desktop that happens to fold",
            "Four hours of battery with the screen at half brightness. Four.",
            "## Battery\n\nThe spec sheet says *up to 14 hours*. My worst case was a video call "
            "plus a browser:\n\n1. 100% at 09:00\n2. 41% at 11:10\n3. Dead at 13:05\n\n"
            "## Keyboard\n\nActually excellent, which makes the rest more frustrating.\n\n"
            "## Should you buy it?\n\nOnly if it lives on a desk. Then buy a desktop.",
            ["laptop", "battery"],
        ),
        (
            "SmartLock 2: smart enough to lock you out",
            "A firmware update bricked the keypad. The physical key worked, which is the only "
            "reason I am writing this indoors.",
            "The app pushed an update overnight. In the morning the keypad lit up, accepted the "
            "code, and did nothing.\n\nSupport's suggestion was to *remove the lock from the door "
            "and reset it*. The reset button is on the inside.\n\n"
            "> A lock whose failure mode is \"use the key\" is a very expensive key.",
            ["smart-home", "security"],
        ),
    ],
    "kai": [
        (
            "Brewmaster 9000 makes coffee and a mess",
            "The drip tray holds exactly one cup less than the carafe.",
            "## The good\n\nThe coffee is fine. Genuinely fine.\n\n"
            "## The bad\n\n- The drip tray overflows on a full carafe. Every. Single. Time.\n"
            "- Descaling mode takes 45 minutes and beeps the whole way through.\n"
            "- The water tank is removable, but the lid is not, so you fill it at an angle.\n\n"
            "## The ugly\n\nThe \"keep warm\" plate is hot enough to scorch the carafe bottom "
            "after 20 minutes.",
            ["coffee", "kitchen"],
        ),
        (
            "Whirl 700 blender: the lid is the weak link",
            "Blends great until the lid gasket gives up at month three.",
            "Month one: smoothies. Month two: smoothies. Month three: smoothie on the ceiling.\n\n"
            "The gasket is **not sold separately**. The replacement lid costs a third of the "
            "blender.",
            ["kitchen", "blender"],
        ),
    ],
}

# (commenter, post author, post index, text)
COMMENTS = [
    ("jace", "mina", 0, "Same experience with the case. Mine looks five years old after one month."),
    ("kai", "mina", 0, "Did you try the firmware from last week? Mine got slightly better. Slightly."),
    ("mina", "jace", 0, "The keyboard comment is painfully accurate."),
    ("kai", "jace", 1, "Our office has three of these. Two are now paperweights."),
    ("mina", "kai", 0, "The beeping descaling mode is a hostage situation."),
]


def main():
    tokens, posts = {}, {}
    for name, email, bio in USERS:
        resp = call("POST", "/user/sign_up", {"email": email, "password": PASSWORD, "display_name": name})
        if "error" in resp:  # already exists from a previous run
            resp = call("POST", "/user/login", {"email": email, "password": PASSWORD})
        tokens[name] = resp["data"]["access_token"]
        call("PUT", "/user", {"email": email, "display_name": name, "biography": bio,
                              "profile_image_url": ""}, tokens[name])
        posts[name] = []
        for title, description, body, tags in POSTS[name]:
            created = call("POST", "/posts", {"title": title, "description": description,
                                              "body": body, "tags": tags}, tokens[name])
            posts[name].append(created["data"]["id"])

    for who, author, index, body in COMMENTS:
        call("POST", f"/posts/{posts[author][index]}/comments", {"body": body}, tokens[who])

    call("POST", "/profile/mina/follow", None, tokens["jace"])
    call("POST", "/profile/kai/follow", None, tokens["jace"])
    call("POST", "/profile/jace/follow", None, tokens["mina"])
    for who in ("jace", "kai"):
        call("POST", f"/posts/{posts['mina'][0]}/favorite", None, tokens[who])
    call("POST", f"/posts/{posts['jace'][0]}/favorite", None, tokens["mina"])
    call("POST", f"/posts/{posts['kai'][0]}/favorite", None, tokens["mina"])

    print(json.dumps({"users": list(tokens), "posts": posts}, indent=2))


if __name__ == "__main__":
    main()
