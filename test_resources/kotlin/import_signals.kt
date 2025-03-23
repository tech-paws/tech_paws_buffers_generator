// GENERATED, DO NOT EDIT
@file:Suppress(
    "unused",
    "MemberVisibilityCanBePrivate",
    "KotlinRedundantDiagnosticSuppress",
)

package com.tech_paws.buffers_generated

import com.tech_paws.buffers.*
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.MutableStateFlow

import kotlinx.coroutines.channels.Channel

object TestRpc {
    private const val SCOPE_ID = "11111111-1111-1111-1111-111111111111"

    var testCurrent: String = ""
        private set
    private val testFlow = MutableStateFlow(testCurrent)

    val test: StateFlow<String> get() = testFlow

    fun consumeSignals(runtime: TechPawsBuffersRpcSignalRuntime) {
        runtime.consumeResult(
            scopeId = SCOPE_ID,
            methodId = 0U,
        ) { reader ->
            val value = String.readFromBuffers(reader)
            testCurrent = value
            testFlow.tryEmit(value)
        }
    }
}
