
import 'package:integration_tests/integration_tests.dart'
    show initialize, add;
import 'package:test/test.dart';

Future<void> main() async {
  await initialize();

  test('add works', () async {
    final sum = await add(19, 20);
    expect(sum, equals(39));
  });
}
