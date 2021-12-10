import 'package:integration_tests/integration_tests.dart' show asyncApi, api2;
import 'package:test/test.dart';

Future<void> main() async {
  test('add works', () async {
    expect(await asyncApi.add(19, 20), equals(39));
  });

  test('sub works', () async {
    expect(await asyncApi.sub(100, 20), equals(80));
  });

  test('api2 works', () async {
    expect(await api2.getTheByte(), equals(12));
  });
}
