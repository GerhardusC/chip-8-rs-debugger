echo -e "\e[32mFetching URLS for games from https://github.com/JohnEarnest/chip8Archive/tree/master/roms\e[0m"
gh api repos/JohnEarnest/chip8Archive/git/trees/master?recursive=1 --jq \
	'.tree[]
		| select(.type == "blob") 	
		| select(.path | startswith("roms/"))
		| "https://raw.githubusercontent.com/JohnEarnest/chip8Archive/refs/heads/master/\(.path)"' \
	| xargs -I {} python3 -c "import urllib.parse; print(urllib.parse.quote('{}', safe=':/\\\?=&'))" \
	| grep \.ch8 \
	> games.txt

echo -e "\e[32mFetching URLS for games and programs from https://github.com/kripod/chip8-roms\e[0m"
gh api repos/kripod/chip8-roms/git/trees/master?recursive=1 --jq \
	'.tree[]
		| select(.type == "blob") 	
		| select(.path | startswith("games/") or startswith("hires/") or startswith("programs/"))
		| "https://raw.githubusercontent.com/kripod/chip8-roms/refs/heads/master/\(.path)"' \
	| xargs -I {} python3 -c "import urllib.parse; print(urllib.parse.quote('{}', safe=':/\\\?=&'))" \
	| grep \.ch8 \
	>> games.txt

