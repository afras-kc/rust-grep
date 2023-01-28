# get the argument message
message="$1"

if [[ -z "${message// }" ]]
    then
        message=$(date '+%Y-%m-%d %H:%M:%S')
fi

git add .
git commit -m "$message"
git push origin 
echo "====DONE"
