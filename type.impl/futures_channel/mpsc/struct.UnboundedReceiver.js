(function() {var type_impls = {
"core_network":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; UnboundedReceiver&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.close\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">close</a>(&amp;mut self)</h4></section></summary><div class=\"docblock\"><p>Closes the receiving half of a channel, without dropping it.</p>\n<p>This prevents any further messages from being sent on the channel while\nstill enabling the receiver to drain messages that are buffered.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_next\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">try_next</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;T&gt;, TryRecvError&gt;</h4></section></summary><div class=\"docblock\"><p>Tries to receive the next message without notifying a context if empty.</p>\n<p>It is not recommended to call this function from inside of a future,\nonly when you’ve otherwise arranged to be notified when the channel is\nno longer empty.</p>\n<p>This function returns:</p>\n<ul>\n<li><code>Ok(Some(t))</code> when message is fetched</li>\n<li><code>Ok(None)</code> when channel is closed and no messages left in the queue</li>\n<li><code>Err(e)</code> when there are no messages available, but channel is not yet closed</li>\n</ul>\n</div></details></div></details>",0,"core_network::ping::HeartbeatGetPongRx"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-FusedStream-for-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-FusedStream-for-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; FusedStream for UnboundedReceiver&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_terminated\" class=\"method trait-impl\"><a href=\"#method.is_terminated\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">is_terminated</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Returns <code>true</code> if the stream should no longer be polled.</div></details></div></details>","FusedStream","core_network::ping::HeartbeatGetPongRx"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-Drop-for-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for UnboundedReceiver&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","core_network::ping::HeartbeatGetPongRx"],["<section id=\"impl-Unpin-for-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-Unpin-for-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for UnboundedReceiver&lt;T&gt;</h3></section>","Unpin","core_network::ping::HeartbeatGetPongRx"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Stream-for-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-Stream-for-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Stream for UnboundedReceiver&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Item\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Item\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Item</a> = T</h4></section></summary><div class='docblock'>Values yielded by the stream.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.poll_next\" class=\"method trait-impl\"><a href=\"#method.poll_next\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">poll_next</a>(\n    self: <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html\" title=\"struct core::pin::Pin\">Pin</a>&lt;&amp;mut UnboundedReceiver&lt;T&gt;&gt;,\n    cx: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html\" title=\"struct core::task::wake::Context\">Context</a>&lt;'_&gt;\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html\" title=\"enum core::task::poll::Poll\">Poll</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;T&gt;&gt;</h4></section></summary><div class='docblock'>Attempt to pull out the next value of this stream, registering the\ncurrent task for wakeup if the value is not yet available, and returning\n<code>None</code> if the stream is exhausted. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.size_hint\" class=\"method trait-impl\"><a href=\"#method.size_hint\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">size_hint</a>(&amp;self) -&gt; (<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt;)</h4></section></summary><div class='docblock'>Returns the bounds on the remaining length of the stream. <a>Read more</a></div></details></div></details>","Stream","core_network::ping::HeartbeatGetPongRx"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-Debug-for-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for UnboundedReceiver&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","core_network::ping::HeartbeatGetPongRx"]],
"core_p2p":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; UnboundedReceiver&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.close\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">close</a>(&amp;mut self)</h4></section></summary><div class=\"docblock\"><p>Closes the receiving half of a channel, without dropping it.</p>\n<p>This prevents any further messages from being sent on the channel while\nstill enabling the receiver to drain messages that are buffered.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_next\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">try_next</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;T&gt;, TryRecvError&gt;</h4></section></summary><div class=\"docblock\"><p>Tries to receive the next message without notifying a context if empty.</p>\n<p>It is not recommended to call this function from inside of a future,\nonly when you’ve otherwise arranged to be notified when the channel is\nno longer empty.</p>\n<p>This function returns:</p>\n<ul>\n<li><code>Ok(Some(t))</code> when message is fetched</li>\n<li><code>Ok(None)</code> when channel is closed and no messages left in the queue</li>\n<li><code>Err(e)</code> when there are no messages available, but channel is not yet closed</li>\n</ul>\n</div></details></div></details>",0,"core_p2p::api::HeartbeaRequestRx"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-FusedStream-for-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-FusedStream-for-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; FusedStream for UnboundedReceiver&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_terminated\" class=\"method trait-impl\"><a href=\"#method.is_terminated\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">is_terminated</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Returns <code>true</code> if the stream should no longer be polled.</div></details></div></details>","FusedStream","core_p2p::api::HeartbeaRequestRx"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-Drop-for-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for UnboundedReceiver&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","core_p2p::api::HeartbeaRequestRx"],["<section id=\"impl-Unpin-for-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-Unpin-for-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for UnboundedReceiver&lt;T&gt;</h3></section>","Unpin","core_p2p::api::HeartbeaRequestRx"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Stream-for-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-Stream-for-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Stream for UnboundedReceiver&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Item\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Item\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Item</a> = T</h4></section></summary><div class='docblock'>Values yielded by the stream.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.poll_next\" class=\"method trait-impl\"><a href=\"#method.poll_next\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">poll_next</a>(\n    self: <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html\" title=\"struct core::pin::Pin\">Pin</a>&lt;&amp;mut UnboundedReceiver&lt;T&gt;&gt;,\n    cx: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html\" title=\"struct core::task::wake::Context\">Context</a>&lt;'_&gt;\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html\" title=\"enum core::task::poll::Poll\">Poll</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;T&gt;&gt;</h4></section></summary><div class='docblock'>Attempt to pull out the next value of this stream, registering the\ncurrent task for wakeup if the value is not yet available, and returning\n<code>None</code> if the stream is exhausted. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.size_hint\" class=\"method trait-impl\"><a href=\"#method.size_hint\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">size_hint</a>(&amp;self) -&gt; (<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt;)</h4></section></summary><div class='docblock'>Returns the bounds on the remaining length of the stream. <a>Read more</a></div></details></div></details>","Stream","core_p2p::api::HeartbeaRequestRx"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-Debug-for-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for UnboundedReceiver&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","core_p2p::api::HeartbeaRequestRx"]],
"core_transport":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; UnboundedReceiver&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.close\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">close</a>(&amp;mut self)</h4></section></summary><div class=\"docblock\"><p>Closes the receiving half of a channel, without dropping it.</p>\n<p>This prevents any further messages from being sent on the channel while\nstill enabling the receiver to drain messages that are buffered.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.try_next\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">try_next</a>(&amp;mut self) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;T&gt;, TryRecvError&gt;</h4></section></summary><div class=\"docblock\"><p>Tries to receive the next message without notifying a context if empty.</p>\n<p>It is not recommended to call this function from inside of a future,\nonly when you’ve otherwise arranged to be notified when the channel is\nno longer empty.</p>\n<p>This function returns:</p>\n<ul>\n<li><code>Ok(Some(t))</code> when message is fetched</li>\n<li><code>Ok(None)</code> when channel is closed and no messages left in the queue</li>\n<li><code>Err(e)</code> when there are no messages available, but channel is not yet closed</li>\n</ul>\n</div></details></div></details>",0,"core_transport::api::HeartbeaRequestRx"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-FusedStream-for-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-FusedStream-for-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; FusedStream for UnboundedReceiver&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.is_terminated\" class=\"method trait-impl\"><a href=\"#method.is_terminated\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">is_terminated</a>(&amp;self) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Returns <code>true</code> if the stream should no longer be polled.</div></details></div></details>","FusedStream","core_transport::api::HeartbeaRequestRx"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Drop-for-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-Drop-for-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html\" title=\"trait core::ops::drop::Drop\">Drop</a> for UnboundedReceiver&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.drop\" class=\"method trait-impl\"><a href=\"#method.drop\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\" class=\"fn\">drop</a>(&amp;mut self)</h4></section></summary><div class='docblock'>Executes the destructor for this type. <a href=\"https://doc.rust-lang.org/nightly/core/ops/drop/trait.Drop.html#tymethod.drop\">Read more</a></div></details></div></details>","Drop","core_transport::api::HeartbeaRequestRx"],["<section id=\"impl-Unpin-for-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-Unpin-for-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for UnboundedReceiver&lt;T&gt;</h3></section>","Unpin","core_transport::api::HeartbeaRequestRx"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Stream-for-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-Stream-for-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; Stream for UnboundedReceiver&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Item\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Item\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a class=\"associatedtype\">Item</a> = T</h4></section></summary><div class='docblock'>Values yielded by the stream.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.poll_next\" class=\"method trait-impl\"><a href=\"#method.poll_next\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">poll_next</a>(\n    self: <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/pin/struct.Pin.html\" title=\"struct core::pin::Pin\">Pin</a>&lt;&amp;mut UnboundedReceiver&lt;T&gt;&gt;,\n    cx: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/task/wake/struct.Context.html\" title=\"struct core::task::wake::Context\">Context</a>&lt;'_&gt;\n) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/task/poll/enum.Poll.html\" title=\"enum core::task::poll::Poll\">Poll</a>&lt;<a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;T&gt;&gt;</h4></section></summary><div class='docblock'>Attempt to pull out the next value of this stream, registering the\ncurrent task for wakeup if the value is not yet available, and returning\n<code>None</code> if the stream is exhausted. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.size_hint\" class=\"method trait-impl\"><a href=\"#method.size_hint\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">size_hint</a>(&amp;self) -&gt; (<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>, <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.usize.html\">usize</a>&gt;)</h4></section></summary><div class='docblock'>Returns the bounds on the remaining length of the stream. <a>Read more</a></div></details></div></details>","Stream","core_transport::api::HeartbeaRequestRx"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-UnboundedReceiver%3CT%3E\" class=\"impl\"><a href=\"#impl-Debug-for-UnboundedReceiver%3CT%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for UnboundedReceiver&lt;T&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/nightly/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/nightly/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/nightly/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","core_transport::api::HeartbeaRequestRx"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()