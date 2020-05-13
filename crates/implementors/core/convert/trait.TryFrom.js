(function() {var implementors = {};
implementors["wasmer_runtime_c_api"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u32.html\">u32</a>&gt; for <a class=\"enum\" href=\"wasmer_runtime_c_api/export/enum.wasmer_import_export_kind.html\" title=\"enum wasmer_runtime_c_api::export::wasmer_import_export_kind\">wasmer_import_export_kind</a>","synthetic":false,"types":["wasmer_runtime_c_api::export::wasmer_import_export_kind"]}];
implementors["wasmer_runtime_core"] = [{"text":"impl&lt;'a&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"wasmer_runtime_core/table/struct.Anyfunc.html\" title=\"struct wasmer_runtime_core::table::Anyfunc\">Anyfunc</a>&lt;'a&gt;&gt; for <a class=\"struct\" href=\"wasmer_runtime_core/instance/struct.DynFunc.html\" title=\"struct wasmer_runtime_core::instance::DynFunc\">DynFunc</a>&lt;'a&gt;","synthetic":false,"types":["wasmer_runtime_core::instance::DynFunc"]},{"text":"impl&lt;'_&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;&amp;'_ <a class=\"enum\" href=\"wasmer_runtime_core/types/enum.Value.html\" title=\"enum wasmer_runtime_core::types::Value\">Value</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i32.html\">i32</a>","synthetic":false,"types":[]},{"text":"impl&lt;'_&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;&amp;'_ <a class=\"enum\" href=\"wasmer_runtime_core/types/enum.Value.html\" title=\"enum wasmer_runtime_core::types::Value\">Value</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.i64.html\">i64</a>","synthetic":false,"types":[]},{"text":"impl&lt;'_&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;&amp;'_ <a class=\"enum\" href=\"wasmer_runtime_core/types/enum.Value.html\" title=\"enum wasmer_runtime_core::types::Value\">Value</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f32.html\">f32</a>","synthetic":false,"types":[]},{"text":"impl&lt;'_&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;&amp;'_ <a class=\"enum\" href=\"wasmer_runtime_core/types/enum.Value.html\" title=\"enum wasmer_runtime_core::types::Value\">Value</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.f64.html\">f64</a>","synthetic":false,"types":[]},{"text":"impl&lt;'_&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;&amp;'_ <a class=\"enum\" href=\"wasmer_runtime_core/types/enum.Value.html\" title=\"enum wasmer_runtime_core::types::Value\">Value</a>&gt; for <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.u128.html\">u128</a>","synthetic":false,"types":[]}];
implementors["wasmer_wasi"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"wasmer_wasi/syscalls/types/struct.__wasi_subscription_t.html\" title=\"struct wasmer_wasi::syscalls::types::__wasi_subscription_t\">__wasi_subscription_t</a>&gt; for <a class=\"struct\" href=\"wasmer_wasi/syscalls/types/struct.WasiSubscription.html\" title=\"struct wasmer_wasi::syscalls::types::WasiSubscription\">WasiSubscription</a>","synthetic":false,"types":["wasmer_wasi::syscalls::types::WasiSubscription"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html\" title=\"trait core::convert::TryFrom\">TryFrom</a>&lt;<a class=\"struct\" href=\"wasmer_wasi/syscalls/types/struct.WasiSubscription.html\" title=\"struct wasmer_wasi::syscalls::types::WasiSubscription\">WasiSubscription</a>&gt; for <a class=\"struct\" href=\"wasmer_wasi/syscalls/types/struct.__wasi_subscription_t.html\" title=\"struct wasmer_wasi::syscalls::types::__wasi_subscription_t\">__wasi_subscription_t</a>","synthetic":false,"types":["wasmer_wasi::syscalls::types::__wasi_subscription_t"]}];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        })()