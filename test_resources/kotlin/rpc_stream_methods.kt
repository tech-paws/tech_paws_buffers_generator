object ExamplesStreamRpc {
    private const val SCOPE_ID = "723ca727-6a66-43a7-bfcc-b8ad94eac9be"

    var counterCurrent: Int = 0
        private set
    private val counterFlow = MutableStateFlow(counterCurrent)
    var themeCurrent: String = ""
        private set
    private val themeFlow = MutableStateFlow(themeCurrent)
    private val triggerFlow = MutableStateFlow(Unit)

    val counter: Flow<Int> get() = counterFlow

    val theme: Flow<String> get() = themeFlow

    val trigger: Flow<Unit> get() = triggerFlow

    fun consumeSignals(runtime: TechPawsBuffersRpcSignalRuntime) {
        runtime.consumeResult(
            scopeId = SCOPE_ID,
            methodId = 0U,
        ) { reader ->
            val value = Int.readFromBuffers(reader)
            counterCurrent = value
            counterFlow.tryEmit(value)
        }
        runtime.consumeResult(
            scopeId = SCOPE_ID,
            methodId = 1U,
        ) { reader ->
            val value = String.readFromBuffers(reader)
            themeCurrent = value
            themeFlow.tryEmit(value)
        }
        runtime.consumeResult(
            scopeId = SCOPE_ID,
            methodId = 2U,
        ) { reader ->
            triggerFlow.tryEmit(Unit)
        }
    }
}
