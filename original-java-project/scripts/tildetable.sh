#!/bin/bash
if [ -z "${BASEDIR}" ]
then
    if [ -d "../src" ]; then BASEDIR=".."
    elif [ -d "./src" ]; then BASEDIR="."
    else
        echo "Couldn't find the source files in the current working directory."
        echo "You can also pass this path in using the BASEDIR environment variable."
        echo "CWD=$(pwd)"
    fi
fi

# Courtesy https://stackoverflow.com/questions/2264428/how-to-convert-a-string-to-lower-case-in-bash
lc() {
    case "$1" in
        [A-Z])
            n=$(printf "%d" "'$1")
            n=$((n+32))
            printf \\$(printf "%o" "$n")
            ;;
        *)
            printf "%s" "$1"
        ;;
    esac
}

# Courtesy https://www.devopsroles.com/bash-script-recursive-file-list/
find_srcfiles_recurse() {
    shopt -s extglob
    for entry in "$1"/*; do
        [[ -f "$entry" ]] && [[ $(lc "${entry: -5}") == ".java" ]] && SRCFILES="${SRCFILES##*( )} ${entry}"
    done
    for entry in "$1"/*; do
        [[ -d "$entry" ]] && find_srcfiles_recurse "$entry"
    done
    shopt -u extglob
}

OUTDIR="${BASEDIR}/out"
SRCDIR="${BASEDIR}/src"
SRCFILES=""
find_srcfiles_recurse $SRCDIR
EXEC_MKOUTDIR="mkdir -v ${OUTDIR}"
EXEC_JAVAC="javac -d ${OUTDIR} --source-path ${SRCDIR} ${SRCFILES}"
EXEC_JAVA="java -cp ${OUTDIR} me/daball/tildetable/Controller $@"

echo "Pre-flight configuration:"
echo "BASEDIR=${BASEDIR}"
echo "OUTDIR=${OUTDIR}"
echo "SRCDIR=${SRCDIR}"
echo "SRCFILES=${SRCFILES}"
echo "EXEC_MKOUTDIR=${EXEC_MKOUTDIR}"
echo "EXEC_JAVAC=${EXEC_JAVAC}"
echo "EXEC_JAVA=${EXEC_JAVA}"
if [ ! -d "${OUTDIR}" ]; then
    echo "Output directory does not exist. Making directory."
    $EXEC_MKOUTDIR
fi
echo "Building Tilde Table app..."
$EXEC_JAVAC
echo "Running Tilde Table app..."
$EXEC_JAVA
