// GENERATED, DO NOT EDIT
@file:Suppress("unused")

package com.tech_paws.buffers_generated

import com.tech_paws.buffers.*

object TestRpc {
    private const val SCOPE_ID = "7bc4e8da-3363-432e-aae5-fcf5bec941dd"

    fun testFn() {
        TechPawsBuffersRpcRuntime.rpc(
            scopeId = SCOPE_ID,
            methodId = 0U,
        ) { runtime ->
            runtime.callRpc()
        }
    }
}
