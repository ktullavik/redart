

void main() {
  var a = 'ra';
  var b = 'da';
  var c = "bra";
  print('ab${a + 'ca${b + '${c}'}'}');
  assert('ab${a + 'ca${b + '${c}'}'}' == 'abracadabra');
}

