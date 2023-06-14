const commands = _Commands();
const addr = _Addr();
const groups = _Groups();
const commandsBuffers = _CommandsBuffers();

class _Commands {
  const _Commands();

  final int drawLines = 131073;
  final int drawPath = 131074;
  final int drawQuads = 131075;
  final int drawCenteredQuads = 131076;
  final int drawTexts = 131077;
  final int setColorPipeline = 131078;
  final int setTexturePipeline = 131079;
  final int drawCircles = 131080;
  final int drawHollowCircles = 131081;
}

class _Addr {
  const _Addr();

  final String someValue = "Hello World!";
  final groups = const _Groups();
  final double deltaTime = 16.6;
  final bool flag = true;
  final commandsBuffers = const _CommandsBuffers();
}

class _Groups {
  const _Groups();

  final int main = 0;
  final int mainRender = 1;
  final int rpc = 2;
  final int rpcSync = 3;
  final int rpcRead = 4;
}

class _CommandsBuffers {
  const _CommandsBuffers();

  final int win1MainRender = 0;
}
