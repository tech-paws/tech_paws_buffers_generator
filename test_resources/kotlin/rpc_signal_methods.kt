object TestRpc {
    private const val SCOPE_ID = "11111111-1111-1111-1111-111111111111"

    var counterCurrent: Int = 0
        private set
    private val counterFlow = MutableStateFlow(counterCurrent)
    var themeCurrent: String = ""
        private set
    private val themeFlow = MutableStateFlow(themeCurrent)
    private val triggerFlow = MutableStateFlow(Unit)

    val counter: StateFlow<Int> get() = counterFlow

    val theme: StateFlow<String> get() = themeFlow

    val trigger: StateFlow<Unit> get() = triggerFlow

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
