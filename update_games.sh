fetch_urls() {
	local repo_name=$1
	local main_branch=$2

	echo -e "\e[32mFetching URLS for games from https://github.com/$repo_name/tree/$main_branch/roms\e[0m" 1>&2

	shift 2
	
	local directories=("${@}")
	local query='.tree[] | select(.type == "blob") | select(.path | '

	local acc=''
	for pathname in "${directories[@]}"; do
		if [ -z "$acc" ]; then
			acc+='startswith("'"$pathname"'/") '
		else
			acc+=' or startswith("'"$pathname"'/") '
		fi
	done
	query+="$acc"
	query+=') | "https://raw.githubusercontent.com/'"$repo_name"'/refs/heads/'"$main_branch"'/\(.path)"'

	gh api repos/$repo_name/git/trees/$main_branch?recursive=1 --jq "$query" \
		| xargs -I {} python3 -c "import urllib.parse; print(urllib.parse.quote('{}', safe=':/\\\?=&'))" \
		| grep \.ch8
}

fetch_urls "Timendus/chip8-test-suite" main bin > games.txt
fetch_urls "JohnEarnest/chip8Archive" master roms >> games.txt
fetch_urls "kripod/chip8-roms" master games hires programs >> games.txt

