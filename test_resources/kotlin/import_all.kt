// GENERATED, DO NOT EDIT
@file:Suppress(
    "unused",
    "MemberVisibilityCanBePrivate",
    "KotlinRedundantDiagnosticSuppress",
)

package com.tech_paws.buffers_generated

import com.tech_paws.buffers.*
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.MutableStateFlow

import kotlinx.coroutines.channels.Channel

object Address {
    const val ADDRESS: ULong = 1UL
}

data class Test(
    val value: Int,
) {
    companion object {
        fun createDefault(): Test = Test(
            value = 0,
        )

        fun readFromBuffers(reader: Long): Test {
            val value = Int.readFromBuffers(reader)

            return Test(
                value = value,
            )
        }

        fun skipInBuffers(reader: Long, count: Int) {
            for (i in 0..<count) {
                Int.readFromBuffers(reader)
            }
        }
    }

    fun writeToBuffers(writer: Long) {
        value.writeToBuffers(writer)
    }
}

object TestRpc {
    private const val SCOPE_ID = "7bc4e8da-3363-432e-aae5-fcf5bec941dd"

    var testCurrent: String = ""
        private set
    private val testFlow = MutableStateFlow(testCurrent)

    val test: Flow<String> get() = testFlow

    fun consumeSignals(runtime: TechPawsBuffersRpcSignalRuntime) {
        runtime.consumeResult(
            scopeId = SCOPE_ID,
            methodId = 1U,
        ) { reader ->
            val value = String.readFromBuffers(reader)
            testCurrent = value
            testFlow.tryEmit(value)
        }
    }

    fun testFn() {
        TechPawsBuffersRpcRuntime.rpc(
            scopeId = SCOPE_ID,
            methodId = 0U,
        ) { runtime ->
            runtime.callRpc()
        }
    }
}
