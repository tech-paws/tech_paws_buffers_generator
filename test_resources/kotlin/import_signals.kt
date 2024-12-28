// GENERATED, DO NOT EDIT
@file:Suppress("unused")

package com.tech_paws.buffers_generated

import com.tech_paws.buffers.*
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow

import kotlinx.coroutines.channels.Channel

object TestRpc {
    private const val SCOPE_ID = "7bc4e8da-3363-432e-aae5-fcf5bec941dd"

    var testCurrent: String = ""
        private set
    private val testFlow = MutableStateFlow(testCurrent)

    val test: Flow<String> get() = testFlow

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
