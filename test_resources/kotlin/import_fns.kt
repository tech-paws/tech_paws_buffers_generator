// GENERATED, DO NOT EDIT
@file:Suppress(
    "unused",
    "MemberVisibilityCanBePrivate",
    "KotlinRedundantDiagnosticSuppress",
)

package com.tech_paws.buffers_generated

import com.tech_paws.buffers.*

object TestRpc {
    private const val SCOPE_ID = "11111111-1111-1111-1111-111111111111"

    fun testFn() {
        TechPawsBuffersRpcRuntime.rpc(
            scopeId = SCOPE_ID,
            methodId = 0U,
        ) { runtime ->
            runtime.callRpc()
        }
    }
}
