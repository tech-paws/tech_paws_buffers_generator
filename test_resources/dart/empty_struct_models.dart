class Empty {
  const Empty();
}

class EmptyBuffersFactory implements BuffersFactory<Empty> {
  const EmptyBuffersFactory();

  Empty createDefault() => const Empty();
}
