APP=./minimp3

set +e
for i in vectors/*.bit; do
$APP $i ${i%.*}.pcm
retval=$?
echo $i exited with code=$retval
if [ ! $retval -eq 0 ]; then
  exit 1
fi
done
