# ripgrep i18n - zh-CN
# 格式：键 = 文本（多行值持续到下一个键行）

flag-after-context-short = 显示每个匹配后的 NUM 行。

flag-after-context-long = 显示每个匹配之后的 \fINUM\fP 行。
.sp
这会覆盖 \flag{passthru} 标志，并部分覆盖 \flag{context} 标志。

flag-auto-hybrid-regex-short = （已弃用）在合适时使用 PCRE2。

flag-auto-hybrid-regex-long = 已弃用。请改用 \flag{engine}。
.sp
使用此标志时，ripgrep 将根据模式中使用的特性，在支持的正则引擎之间动态选择。当 ripgrep 选择一个正则引擎后，该选择将应用于提供给 ripgrep 的每个正则表达式（例如，通过多个 \flag{regexp} 或 \flag{file} 标志）。
.sp
作为此标志行为的一个示例：只要模式能用 ripgrep 默认的基于有限自动机的正则引擎成功编译，ripgrep 就会尝试使用该引擎。如果启用了 PCRE2，且给定的模式无法用默认正则引擎编译，那么将自动使用 PCRE2 进行搜索。如果 PCRE2 不可用，那么此标志没有任何效果，因为只有一个正则引擎可选。
.sp
将来，ripgrep 可能会调整其决定使用哪个正则引擎的启发式规则。一般来说，启发式规则将限于对模式的静态分析，而不会涉及搜索文件时观察到的任何特定运行时行为。
.sp
使用此标志的主要缺点是：可能并不总是显而易见 ripgrep 使用了哪个正则引擎，因此 ripgrep 的匹配语义或性能特征可能会发生微妙且意外的变化。不过，在许多情况下，所有正则引擎都会对什么是匹配达成一致，而且能够透明地支持环视（look-around）和反向引用（backreferences）等更高级的正则特性（而无需显式启用它们）是很好的。

flag-before-context-short = 显示每个匹配前的 NUM 行。

flag-before-context-long = 显示每个匹配之前的 \fINUM\fP 行。
.sp
这会覆盖 \flag{passthru} 标志，并部分覆盖 \flag{context} 标志。

flag-binary-short = 搜索二进制文件。

flag-binary-long = 启用此标志将使 ripgrep 搜索二进制文件。默认情况下，ripgrep 会尝试自动跳过二进制文件，以提高结果的相关性并加快搜索速度。
.sp
二进制文件是根据其是否包含 \fBNUL\fP 字节进行启发式检测的。默认情况下（未设置此标志时），一旦看到 \fBNUL\fP 字节，ripgrep 就会停止搜索该文件。通常，\fBNUL\fP 字节出现在大多数二进制文件的开头。如果 \fBNUL\fP 字节出现在匹配之后，那么 ripgrep 将不会打印该匹配，停止搜索该文件，并发出警告说明某些匹配被抑制了。
.sp
相反，当提供此标志时，即使发现 \fBNUL\fP 字节，ripgrep 也会继续搜索文件。特别是，如果发现 \fBNUL\fP 字节，ripgrep 将继续搜索，直到找到匹配或到达文件末尾（以先到者为准）。如果找到匹配，ripgrep 将停止并打印警告，说明搜索提前停止了。
.sp
如果您希望 ripgrep 完全不做任何特殊的 \fBNUL\fP 字节处理（并可能将二进制数据打印到 stdout），那么您应该使用 \flag{text} 标志。
.sp
\flag{binary} 标志用于控制 ripgrep 的自动过滤机制。因此，在显式搜索文件或搜索 stdin 时，不需要使用它。也就是说，它只在递归搜索目录时适用。
.sp
当第三次提供 \flag{unrestricted} 标志时，此标志会自动启用。
.sp
此标志覆盖 \flag{text} 标志。

flag-block-buffered-short = 强制块缓冲。

flag-block-buffered-long = 启用后，ripgrep 将使用块缓冲。也就是说，每当找到匹配行时，它会被写入内存缓冲区，并且直到缓冲区达到一定大小才会被写入 stdout。当 ripgrep 的 stdout 被重定向到管道或文件时，这是默认行为。当 ripgrep 的 stdout 连接到 tty 时，默认使用行缓冲。在向 tty 倾倒大量内容时，强制块缓冲可能很有用。
.sp
这会覆盖 \flag{line-buffered} 标志。

flag-byte-offset-short = 打印每个匹配行的字节偏移量。

flag-byte-offset-long = 在每行输出之前打印输入文件内从 0 开始的字节偏移量。如果指定了 \flag{only-matching}，则打印匹配文本本身的偏移量。
.sp
如果 ripgrep 执行了转码，那么字节偏移量是以转码结果为基准的，而不是原始数据。这也同样适用于对数据的其他转换，例如解压缩或 \flag{pre} 过滤器。

flag-case-sensitive-short = 区分大小写搜索（默认）。

flag-case-sensitive-long = 区分大小写执行搜索。这是默认模式。
.sp
这是一个全局选项，适用于给予 ripgrep 的所有模式。单个模式仍然可以通过内联正则标志进行不区分大小写的匹配。例如，即使使用了此标志，\fB(?i)abc\fP 也会不区分大小写地匹配 \fBabc\fP。
.sp
此标志覆盖 \flag{ignore-case} 和 \flag{smart-case} 标志。

flag-color-short = 何时使用颜色。

flag-color-long = 此标志控制何时使用颜色。默认设置为 \fBauto\fP，这意味着 ripgrep 会尝试猜测何时使用颜色。例如，如果 ripgrep 打印到 tty，那么它会使用颜色；但如果它被重定向到文件或管道，那么它会抑制颜色输出。
.sp
ripgrep 在其他一些情况下也会默认抑制颜色输出。这些情况包括但不限于：
.sp
.IP \(bu 3n
当 \fBTERM\fP 环境变量未设置或设置为 \fBdumb\fP 时。
.sp
.IP \(bu 3n
当设置了 \fBNO_COLOR\fP 环境变量时（无论其值是什么）。
.sp
.IP \(bu 3n
当给出隐含不需要颜色的标志时。例如，\flag{vimgrep} 和 \flag{json}。
.
.PP
此标志的可能值为：
.sp
.IP \fBnever\fP 10n
绝不使用颜色。
.sp
.IP \fBauto\fP 10n
默认值。ripgrep 会尽量智能。
.sp
.IP \fBalways\fP 10n
无论输出发送到哪里，始终使用颜色。
.sp
.IP \fBansi\fP 10n
与 'always' 类似，但发出 ANSI 转义序列（即使在 Windows 控制台中也是如此）。
.
.PP
此标志还控制是否发出超链接。例如，当指定了超链接格式时，如果颜色被抑制，就不会使用超链接。如果希望发出超链接但不使用颜色，那么必须使用 \flag{colors} 标志手动将所有颜色样式设置为 \fBnone\fP：
.sp
.EX
    \-\-colors 'path:none' \\
    \-\-colors 'line:none' \\
    \-\-colors 'column:none' \\
    \-\-colors 'match:none' \\
    \-\-colors 'highlight:none'
.EE
.sp

flag-colors-short = 配置颜色设置和样式。

flag-colors-long = 此标志指定输出中使用的颜色设置。此标志可以多次提供。设置会迭代应用。预定义的颜色标签限于八种选择之一：\fBred\fP、\fBblue\fP、\fBgreen\fP、\fBcyan\fP、\fBmagenta\fP、\fByellow\fP、\fBwhite\fP 和 \fBblack\fP。样式限于 \fBnobold\fP、\fBbold\fP、\fBnointense\fP、\fBintense\fP、\fBnounderline\fP、\fBunderline\fP、\fBnoitalic\fP 或 \fBitalic\fP。
.sp
该标志的格式为 \fB{\fP\fItype\fP\fB}:{\fP\fIattribute\fP\fB}:{\fP\fIvalue\fP\fB}\fP。\fItype\fP 应为 \fBpath\fP、\fBline\fP、\fBcolumn\fP、\fBhighlight\fP 或 \fBmatch\fP 之一。\fIattribute\fP 可以是 \fBfg\fP、\fBbg\fP 或 \fBstyle\fP。\fIvalue\fP 是颜色（对于 \fBfg\fP 和 \fBbg\fP）或文本样式。特殊格式 \fB{\fP\fItype\fP\fB}:none\fP 将清除 \fItype\fP 的所有颜色设置。
.sp
例如，以下命令会将匹配颜色更改为品红色，并将行号的背景颜色更改为黄色：
.sp
.EX
    rg \-\-colors 'match:fg:magenta' \-\-colors 'line:bg:yellow'
.EE
.sp
另一个示例，以下命令将"高亮"匹配行中的非匹配文本：
.sp
.EX
    rg \-\-colors 'highlight:bg:yellow' \-\-colors 'highlight:fg:black'
.EE
.sp
"highlight" 颜色类型对于对比匹配行与 \flag{before-context}、\flag{after-context}、\flag{context} 或 \flag{passthru} 标志打印的周围上下文特别有用。
.sp
当 tty 支持 ANSI 颜色序列时，\fIvalue\fP 可以使用扩展颜色。这些颜色可以指定为 \fIx\fP（256 色）或
.IB x , x , x
（24 位真彩色），其中 \fIx\fP 是 \fB0\fP 到 \fB255\fP（含）之间的数字。\fIx\fP 可以是一般的十进制数字，也可以是十六进制数字（以 \fB0x\fP 为前缀）。
.sp
例如，以下命令会将匹配的背景颜色更改为 rgb 值 (0,128,255) 所表示的颜色：
.sp
.EX
    rg \-\-colors 'match:bg:0,128,255'
.EE
.sp
或者，等价地，
.sp
.EX
    rg \-\-colors 'match:bg:0x0,0x80,0xFF'
.EE
.sp
请注意，与这些扩展颜色代码一起使用时，\fBintense\fP 和 \fBnointense\fP 样式将没有任何效果。

flag-column-short = 显示列号。

flag-column-long = 显示列号（从 1 开始）。这只会显示每行第一个匹配的列号。这不会尝试考虑 Unicode。一个字节等于一列。这隐含了 \flag{line-number}。
.sp
当使用 \flag{only-matching} 时，写入的列号对应于每个匹配的开始位置。

flag-context-short = 显示每个匹配前后各 NUM 行。

flag-context-long = 显示每个匹配前后各 \fINUM\fP 行。这等同于同时提供具有相同值的 \flag{before-context} 和 \flag{after-context} 标志。
.sp
这会覆盖 \flag{passthru} 标志。\flag{after-context} 和 \flag{before-context} 标志都会部分覆盖此标志，无论顺序如何。例如，\fB\-A2 \-C1\fP 等同于 \fB\-A2 \-B1\fP。

flag-context-separator-short = 设置上下文块的隔符。

flag-context-separator-long = 用于在输出中分隔不连续上下文行的字符串。仅当使用某个上下文标志（即 \flag{after-context}、\flag{before-context} 或 \flag{context}）时才会使用它。可以使用诸如 \fB\\x7F\fP 或 \fB\\t\fP 之类的转义序列。默认值为 \fB\-\-\fP。
.sp
当上下文分隔符设置为空字符串时，仍然会插入一个换行符。要完全禁用上下文分隔符，请使用 \flag-negate{context-separator} 标志。

flag-count-short = 显示每个文件的匹配行数。

flag-count-long = 此标志抑制正常输出，并显示每个被搜索文件中匹配给定模式的行数。每个包含匹配的文件，其路径和计数打印在一行上。请注意，除非启用了 \flag{multiline} 且给定的模式可以跨多行匹配，否则这报告的是匹配的行数，而不是匹配的总数。当启用多行模式且给定的模式可以跨多行匹配时，\flag{count} 等同于 \flag{count-matches}。
.sp
如果只给 ripgrep 一个文件，那么只有存在匹配时才打印计数。在这种情况下，可以使用 \flag{with-filename} 标志强制打印文件路径。如果您需要无论是否存在匹配都打印计数，那么请使用 \flag{include-zero}。
.sp
请注意，此标志的结果可能与 \flag{files-with-matches} 的输出不一致。特别是，默认情况下，ripgrep 会尽量避免搜索包含二进制数据的文件。使用此标志时，ripgrep 需要搜索文件的全部内容，其中可能包含二进制数据。但使用 \flag{files-with-matches} 时，ripgrep 一旦观察到匹配就可以停止，这可能远在遇到任何二进制数据之前。要避免这种不一致而又不禁用二进制检测，请使用 \flag{binary} 标志。
.sp
这会覆盖 \flag{count-matches} 标志。请注意，当 \flag{count} 与 \flag{only-matching} 结合使用时，ripgrep 的行为就像给出了 \flag{count-matches} 一样。

flag-count-matches-short = 显示每个文件的每个匹配的计数。

flag-count-matches-long = 此标志抑制正常输出，并显示每个被搜索文件中给定模式的单个匹配数量。每个包含匹配的文件，其路径和匹配计数打印在一行上。请注意，这报告的是单个匹配的总数，而不是匹配的行数。
.sp
如果只给 ripgrep 一个文件，那么只有存在匹配时才打印计数。在这种情况下，可以使用 \flag{with-filename} 标志强制打印文件路径。
.sp
这会覆盖 \flag{count} 标志。请注意，当 \flag{count} 与 \flag{only-matching} 结合使用时，ripgrep 的行为就像给出了 \flag{count-matches} 一样。

flag-crlf-short = 使用 CRLF 行终止符（适合 Windows）。

flag-crlf-long = 启用后，ripgrep 会将 CRLF（\fB\\r\\n\fP）视为行终止符，而不仅仅是 \fB\\n\fP。
.sp
主要是，这允许正则模式中的行锚点断言 \fB^\fP 和 \fB$\fP 将 CRLF、CR 或 LF 视为行终止符，而不仅仅是 LF。请注意，它们永远不会在 CR 和 LF 之间匹配。CRLF 被视为单个行终止符。
.sp
使用默认正则引擎时，也可以在模式内使用 \fBR\fP 标志启用 CRLF 支持。例如，\fB(?R:$)\fP 将在 CR 或 LF 之前匹配，但永远不会在 CR 和 LF 之间匹配。
.sp
此标志覆盖 \flag{null-data}。

flag-debug-short = 显示调试消息。

flag-debug-long = 显示调试消息。提交错误报告时请使用此标志。
.sp
\flag{debug} 标志通常有助于弄清楚为什么 ripgrep 跳过了对特定文件的搜索。调试消息应该会提到所有被跳过的文件及其被跳过原因。
.sp
要获得更多调试输出，请使用 \flag{trace} 标志，它隐含 \flag{debug} 以及额外的跟踪数据。

flag-dfa-size-limit-short = 正则 DFA 的大小上限。

flag-dfa-size-limit-long = 正则 DFA 的大小上限。默认限制对于任何单个模式或许多较小的模式来说都是相当宽裕的。只有在非常大的正则输入上才应更改此值，因为如果达到限制，可能会改用（较慢的）回退正则引擎。
.sp
输入格式接受 \fBK\fP、\fBM\fP 或 \fBG\fP 后缀，分别对应千字节、兆字节和千兆字节。如果没有提供后缀，则输入按字节处理。

flag-encoding-short = 指定待搜索文件的文本编码。

flag-encoding-long = 指定 ripgrep 在所有被搜索文件上使用的文本编码。默认值为 \fBauto\fP，这将使 ripgrep 在逐文件的基础上尽力自动检测编码。此情况下的自动检测仅适用于以 UTF-8 或 UTF-16 字节顺序标记（BOM）开头的文件。不执行其他自动检测。也可以指定 \fBnone\fP，这将完全禁用 BOM 嗅探，并且总是导致搜索原始字节，包括 BOM（如果存在），无论其编码如何。
.sp
其他支持的值可以在此处的标签列表中找到：
\fIhttps://encoding.spec.whatwg.org/#concept-encoding-get\fP。
.sp
有关编码以及 ripgrep 如何处理编码的更多详细信息，请参阅 \fBGUIDE.md\fP。
.sp
ripgrep 使用的编码检测可以通过 \flag-negate{encoding} 标志恢复为其自动模式。

flag-engine-short = 指定使用哪个正则引擎。

flag-engine-long = 指定使用哪个正则表达式引擎。当您选择一个正则引擎时，该选择将应用于提供给 ripgrep 的每个正则表达式（例如，通过多个 \flag{regexp} 或 \flag{file} 标志）。
.sp
可接受的值是 \fBdefault\fP、\fBpcre2\fP 或 \fBauto\fP。
.sp
默认值为 \fBdefault\fP，它通常最快，并且应该适合大多数用例。\fBpcre2\fP 引擎在您想使用环视或反向引用等特性时通常很有用。\fBauto\fP 将根据模式中使用的特性，在支持的正则引擎之间尽力动态选择。
.sp
请注意，\fBpcre2\fP 引擎是 ripgrep 的可选特性。如果您的 ripgrep 构建中没有包含 PCRE2，那么使用此标志将导致 ripgrep 打印错误消息并退出。
.sp
这会覆盖之前对 \flag{pcre2} 和 \flag{auto-hybrid-regex} 标志的使用。

flag-field-context-separator-short = 设置字段上下文分隔符。

flag-field-context-separator-long = 设置字段上下文分隔符。此分隔符仅在打印上下文行时使用。它用于分隔文件路径、行号、列号和上下文行本身。分隔符可以是任意字节数，包括零个。可以使用诸如 \fB\\x7F\fP 或 \fB\\t\fP 之类的转义序列。
.sp
\fB-\fP 字符是默认值。

flag-field-match-separator-short = 设置字段匹配分隔符。

flag-field-match-separator-long = 设置字段匹配分隔符。此分隔符仅在打印匹配行时使用。它用于分隔文件路径、行号、列号和匹配行本身。分隔符可以是任意字节数，包括零个。可以使用诸如 \fB\\x7F\fP 或 \fB\\t\fP 之类的转义序列。
.sp
\fB:\fP 字符是默认值。

flag-file-short = 从给定文件中搜索模式。

flag-file-long = 从给定文件中搜索模式，每行一个模式。当多次使用此标志或与 \flag{regexp} 标志结合使用时，将搜索所有提供的模式。空模式行将匹配所有输入行，换行符不计入模式。
.sp
当且仅当一行至少匹配其中一个模式时，才会打印该行。
.sp
当 \fIPATTERNFILE\fP 为 \fB-\fP 时，将从 \fBstdin\fP 读取模式。
.sp
当使用 \flag{file} 或 \flag{regexp} 时，ripgrep 将所有位置参数视为要搜索的文件或目录。

flag-files-short = 打印每个将被搜索的文件。

flag-files-long = 打印每个将被搜索的文件，而无需实际执行搜索。这对于确定某个特定文件是否被搜索很有用。
.sp
这会覆盖 \flag{type-list}。

flag-files-with-matches-short = 打印至少有一个匹配的路径。

flag-files-with-matches-long = 只打印至少有一个匹配的路径，并抑制匹配内容。
.sp
请注意，此标志的结果可能与 \flag{count} 的输出不一致。特别是，默认情况下，ripgrep 会尽量避免搜索包含二进制数据的文件。使用此标志时，ripgrep 可能会在观察到二进制数据之前停止搜索。但使用 \flag{count} 时，ripgrep 必须搜索全部内容以确定匹配计数，这意味着它可能会看到导致其跳过搜索该文件的二进制数据。要避免这种不一致而又不禁用二进制检测，请使用 \flag{binary} 标志。
.sp
这会覆盖 \flag{files-without-match}。

flag-files-without-match-short = 打印包含零个匹配的路径。

flag-files-without-match-long = 打印包含零个匹配的路径，并抑制匹配内容。
.sp
这会覆盖 \flag{files-with-matches}。

flag-fixed-strings-short = 将所有模式视为字面量。

flag-fixed-strings-long = 将所有模式视为字面量，而不是正则表达式。使用此标志时，诸如 \fB.(){}*+\fP 之类的特殊正则表达式元字符不需要转义。

flag-follow-short = 跟随符号链接。

flag-follow-long = 此标志指示 ripgrep 在遍历目录时跟随符号链接。默认情况下禁用此行为。请注意，ripgrep 会检查符号链接循环，如果发现循环会报告错误。ripgrep 也会报告损坏链接的错误。要抑制错误消息，请使用 \flag{no-messages} 标志。

flag-generate-short = 生成手册页和补全脚本。

flag-generate-long = 此标志指示 ripgrep 生成由 \fIKIND\fP 标识的某种特殊输出，然后在不搜索的情况下退出。\fIKIND\fP 可以是以下值之一：
.sp
.TP 15
\fBman\fP
以 \fBroff\fP 格式生成 ripgrep 的手册页。
.TP 15
\fBcomplete\-bash\fP
为 \fBbash\fP shell 生成补全脚本。
.TP 15
\fBcomplete\-zsh\fP
为 \fBzsh\fP shell 生成补全脚本。
.TP 15
\fBcomplete\-fish\fP
为 \fBfish\fP shell 生成补全脚本。
.TP 15
\fBcomplete\-powershell\fP
为 PowerShell 生成补全脚本。
.PP
输出写入 \fBstdout\fP。上面的列表可能会随时间扩展。

flag-glob-short = 包含或排除文件路径。

flag-glob-long = 包含或排除匹配给定 glob 的待搜索文件和目录。这总是覆盖任何其他忽略逻辑。可以使用多个 glob 标志。glob 规则匹配 \fB.gitignore\fP glob。在 glob 前加上 \fB!\fP 以排除它。如果多个 glob 匹配一个文件或目录，则命令行中较后给出的 glob 优先。
.sp
作为扩展，glob 支持指定替代项：
.BI "\-g '" ab{c,d}* '
等同于
.BI "\-g " "abc " "\-g " abd.
目前不支持像
.BI "\-g '" ab{,c} '
这样的空替代项。请注意，此语法扩展目前在 \fBgitignore\fP 文件中也是启用的，尽管 git 本身不支持此语法。ripgrep 可能会在 gitignore 文件中禁用此语法扩展，但它始终可以通过 \flag{glob} 标志使用。
.sp
当设置此标志时，每个文件和目录都会应用于它以测试是否匹配。例如，如果您只想在特定目录 \fIfoo\fP 中搜索，那么
.BI "\-g " foo
是不正确的，因为 \fIfoo/bar\fP 不匹配 glob \fIfoo\fP。相反，您应该使用
.BI "\-g '" foo/** '.

flag-glob-case-insensitive-short = 不区分大小写地处理所有 glob 模式。

flag-glob-case-insensitive-long = 不区分大小写地处理所有通过 \flag{glob} 标志给出的 glob 模式。这实际上将 \flag{glob} 视为 \flag{iglob}。

flag-heading-short = 按文件分组打印匹配。

flag-heading-long = 此标志在每个文件的匹配簇上方打印文件路径，而不是将文件路径作为每个匹配行的前缀打印。
.sp
这是打印到 tty 时的默认模式。
.sp
当 \fBstdout\fP 不是 tty 时，ripgrep 将默认采用标准的类似 grep 的格式。在类 Unix 环境中，可以通过将 ripgrep 的输出通过管道传给 \fBcat\fP 来强制使用此格式。例如，\fBrg\fP \fIfoo\fP \fB| cat\fP。

flag-help-short = 显示帮助输出。

flag-help-long = 此标志打印 ripgrep 的帮助输出。
.sp
与大多数其他标志不同，短标志 \fB\-h\fP 和长标志 \fB\-\-help\fP 的行为是不同的。短标志将显示精简的帮助输出，而长标志将显示详细的帮助输出。详细的帮助输出包含完整的文档，而精简的帮助输出只对每个标志显示一行。

flag-hidden-short = 搜索隐藏文件和目录。

flag-hidden-long = 搜索隐藏文件和目录。默认情况下，隐藏文件和目录会被跳过。请注意，如果隐藏文件或目录在忽略文件中被白名单列出，那么即使不提供此标志也会被搜索。同样，如果隐藏文件或目录被显式作为参数给予 ripgrep，也是如此。
.sp
如果文件或目录的基本名称以点字符（\fB.\fP）开头，则认为它是隐藏的。在支持"隐藏"文件属性的操作系统（如 Windows）上，具有此属性的文件也被视为隐藏。
.sp
请注意，\flag{hidden} 将包含诸如 \fB.git\fP 之类的文件和文件夹，无论 \flag{no-ignore-vcs} 如何。要在使用 \flag{hidden} 时排除此类路径，您必须使用另一个标志或忽略文件显式忽略它们。

flag-hostname-bin-short = 运行程序以获取本系统的主机名。

flag-hostname-bin-long = 此标志控制 ripgrep 如何确定本系统的主机名。该标志的值应对应一个可执行文件（可以是路径，也可以是可以通过系统的 \fBPATH\fP 环境变量找到的东西）。设置后，ripgrep 将不带参数运行此可执行文件，并将其输出（去除首尾空白）视为您系统的主机名。
.sp
未设置时（默认值或空字符串），ripgrep 将尝试自动检测您系统的主机名。在 Unix 上，这对应于调用 \fBgethostname\fP。在 Windows 上，这对应于调用 \fBGetComputerNameExW\fP 以获取系统的"物理 DNS 主机名"。
.sp
ripgrep 使用您系统的主机名来生成超链接。

flag-hyperlink-format-short = 设置超链接的格式。

flag-i-glob-short = 不区分大小写地包含/排除路径。

flag-i-glob-long = 包含或排除匹配给定 glob 的待搜索文件和目录。这总是覆盖任何其他忽略逻辑。可以使用多个 glob 标志。glob 规则匹配 \fB.gitignore\fP glob。在 glob 前加上 \fB!\fP 以排除它。如果多个 glob 匹配一个文件或目录，则命令行中较后给出的 glob 优先。通过此标志使用的 glob 不区分大小写地进行匹配。

flag-ignore-case-short = 不区分大小写搜索。

flag-ignore-case-long = 提供此标志时，所有模式都将不区分大小写地进行搜索。ripgrep 默认正则引擎使用的不区分大小写规则符合 Unicode 的"简单"大小写折叠规则。
.sp
这是一个全局选项，适用于给予 ripgrep 的所有模式。单个模式仍然可以通过内联正则标志进行区分大小写的匹配。例如，即使使用了此标志，\fB(?\-i)abc\fP 也会区分大小写地匹配 \fBabc\fP。
.sp
此标志覆盖 \flag{case-sensitive} 和 \flag{smart-case}。

flag-ignore-file-short = 指定额外的忽略文件。

flag-ignore-file-long = 指定一个或多个 \fBgitignore\fP 格式规则文件的路径。这些模式在 \fB.gitignore\fP、\fB.rgignore\fP 和 \fB.ignore\fP 中的模式应用之后应用，并相对于当前工作目录进行匹配。也就是说，通过此标志指定的文件比在目录树中自动找到的文件优先级低。可以通过重复使用此标志来指定多个额外的忽略文件。指定多个忽略文件时，较早的文件比较晚的文件优先级低。
.sp
如果您正在寻找一种直接在命令行上包含或排除文件和目录的方法，那么请改用 \flag{glob}。

flag-ignore-file-case-insensitive-short = 不区分大小写地处理忽略文件。

flag-ignore-file-case-insensitive-long = 不区分大小写地处理忽略文件（\fB.gitignore\fP、\fB.ignore\fP 等）。请注意，这会带来性能损失，并且在不区分大小写的文件系统（如 Windows）上最有用。

flag-include-zero-short = 在摘要输出中包含零匹配。

flag-include-zero-long = 与 \flag{count} 或 \flag{count-matches} 一起使用时，这会使 ripgrep 打印每个文件的匹配数，即使有零个匹配也是如此。默认情况下这是禁用的，但可以启用以使 ripgrep 的行为更像 grep。

flag-index-short = 在可用时使用搜索索引。

flag-index-long = 启用使用索引进行搜索。没有此标志时，ripgrep 从不使用索引。
.sp
ripgrep 按以下顺序查找索引。第一个找到的一个或多个索引的步骤胜出：
.sp
.IP 1. 4
当命令行上给出路径操作数时，每个操作数都被解释为索引目录，并按命令行顺序搜索每个索引。不是有效索引的操作数是错误。
.sp
.IP 2. 4
由 \fBRIPGREP_INDEX_PATH\fP 环境变量命名的索引。
.sp
.IP 3. 4
当前工作目录中 \fB.ripgrep\fP 目录里的有效索引。
.sp
.IP 4. 4
当前工作目录最近的父目录中 \fB.ripgrep\fP 目录里的有效索引。
.PP
如果没有找到索引，ripgrep 会执行普通搜索。
.sp
索引候选按显式 glob 和文件类型选择、隐藏文件与深度设置以及最大文件大小进行过滤。在索引搜索期间，不读取也不重新应用忽略文件（包括 \fB.gitignore\fP）。需要转换内容或每个文件的选项会使查询不符合候选过滤的条件，从而触发下面的回退。
.sp
此标志最多可以给出两次。当给出一次且查询无法使用索引时，ripgrep 会执行普通搜索。当给出两次且找到了索引时，如果查询无法使用该索引，ripgrep 会停止而不是执行普通搜索。

flag-index-crud-short = 创建或更新搜索索引。

flag-index-crud-long = 为命令行上给出的文件和目录创建或更新索引。未给出路径时，这会递归地为当前工作目录建立索引。添加到索引中的文件恰好是 ripgrep 正常遍历和过滤选项所选择的那些文件。
.sp
只有当现有文件的修改时间晚于其被索引的时间时，才会重新索引该文件。使用 \flag{x-force} 重新索引每个选定的文件，包括文件系统上修改时间不可靠或被故意保留的文件。先前已索引但不再可访问的路径会从索引中移除。
.sp
ripgrep 按以下顺序选择索引位置：
.sp
.IP 1. 4
\flag{x-path} 给出的路径。
.sp
.IP 2. 4
\fBRIPGREP_INDEX_PATH\fP 环境变量中的路径。
.sp
.IP 3. 4
当前工作目录中 \fB.ripgrep\fP 目录里的有效索引。
.sp
.IP 4. 4
当前工作目录最近的父目录中 \fB.ripgrep\fP 目录里的有效索引。
.sp
.IP 5. 4
当前工作目录中的 \fB.ripgrep\fP 目录，必要时会创建它。
.PP
如果最终位置已存在但不包含有效索引，那么 ripgrep 会报告错误。
.sp
例如，这会为当前目录创建或增量更新索引：
.sp
.EX
    rg --x-crud
.EE
.sp
这会在同一索引中更新一个路径：
.sp
.EX
    rg --x-crud path/to/file
.EE

flag-index-force-short = 强制重新索引所选文件。

flag-index-force-long = 与 \flag{x-crud} 一起使用时，即使文件的修改时间表明索引已是最新，也会重新索引每个选定的文件。

flag-index-path-short = 设置要更新的索引的路径。

flag-index-path-long = 设置 \flag{x-crud} 使用的索引路径。这优先于 \fBRIPGREP_INDEX_PATH\fP 和 \fB.ripgrep\fP 目录的自动发现。

flag-invert-match-short = 反转匹配。

flag-invert-match-long = 此标志反转匹配。也就是说，ripgrep 将打印不匹配的行，而不是打印匹配的行。
.sp
请注意，这只会反转逐行匹配。例如，将此标志与 \flag{files-with-matches} 结合使用将输出包含任何不匹配给定模式的行的文件。这与 \flag{files-without-match} 不同，后者输出的是不包含任何匹配行的文件。

flag-json-short = 以 JSON Lines 格式显示搜索结果。

flag-json-long = 启用以 JSON Lines 格式打印结果。
.sp
提供此标志时，ripgrep 将发出一系列消息，每条消息编码为一个 JSON 对象，共有五种不同的消息类型：
.sp
.TP 12
\fBbegin\fP
表示正在搜索一个文件且该文件至少包含一个匹配的消息。
.TP 12
\fBend\fP
表示文件搜索已完成的消息。此消息还包含关于特定文件搜索的汇总统计信息。
.TP 12
\fBmatch\fP
表示找到匹配的消息。这包括匹配的文本和偏移量。
.TP 12
\fBcontext\fP
表示找到上下文行的消息。这包括行的文本，以及（如果搜索被反转）任何匹配信息。
.TP 12
\fBsummary\fP
ripgrep 发出的最后一条消息，包含关于跨所有文件的搜索的汇总统计信息。
.PP
由于文件路径或文件内容不能保证是有效的 UTF-8，而 JSON 本身必须可以用 Unicode 编码表示，ripgrep 会将所有数据元素作为具有两个键之一的对象发出：\fBtext\fP 或 \fBbytes\fP。当数据是有效的 UTF-8 时，\fBtext\fP 是普通的 JSON 字符串，而 \fBbytes\fP 是数据的 base64 编码内容。
.sp
JSON Lines 格式仅支持用于显示搜索结果。它不能与发出其他类型输出的标志一起使用，例如 \flag{files}、\flag{files-with-matches}、\flag{files-without-match}、\flag{count} 或 \flag{count-matches}。如果上述任一标志与 \flag{json} 一起使用，ripgrep 将报告错误。
.sp
其他控制标准输出方面的标志，如 \flag{only-matching}、\flag{heading}、\flag{replace}、\flag{max-columns} 等，在设置 \flag{json} 时没有效果。但是，启用 JSON 输出将始终隐式且无条件地启用 \flag{stats}。
.sp
所用 JSON 格式的更完整描述可以在这里找到：
\fIhttps://docs.rs/grep-printer/*/grep_printer/struct.JSON.html\fP。

flag-line-buffered-short = 强制行缓冲。

flag-line-buffered-long = 启用后，ripgrep 将始终使用行缓冲。也就是说，每当找到匹配行时，它会立即刷新到 stdout。当 ripgrep 的 stdout 连接到 tty 时，这是默认行为；但在其他情况下，ripgrep 会使用块缓冲，这通常更快。此标志强制 ripgrep 使用行缓冲，即使它本来会使用块缓冲。这在 shell 管道中通常很有用，例如：
.sp
.EX
    tail -f something.log | rg foo --line-buffered | rg bar
.EE
.sp
这会覆盖 \flag{block-buffered} 标志。

flag-line-number-short = 显示行号。

flag-line-number-long = 显示行号（从 1 开始）。
.sp
当 stdout 连接到 tty 时，默认启用。
.sp
此标志可以通过 \flag{no-line-number} 禁用。

flag-line-number-no-short = 隐藏行号。

flag-line-number-no-long = 抑制行号。
.sp
当 stdout 未连接到 tty 时，行号默认关闭。
.sp
可以通过 \flag{line-number} 强制开启行号。

flag-line-regexp-short = 显示以行边界包围的匹配。

flag-line-regexp-long = 启用后，ripgrep 将只显示以行边界包围的匹配。这等同于将每个模式用 \fB^\fP 和 \fB$\fP 包围。换句话说，这只打印整行参与匹配的行。
.sp
这会覆盖 \flag{word-regexp} 标志。

flag-max-columns-short = 省略超过此限制的行。

flag-max-columns-long = 给出后，ripgrep 将省略超过此字节限制的行。不打印长行，而只打印该行中匹配的数量。
.sp
当省略此标志或将其设置为 \fB0\fP 时，它没有任何效果。

flag-max-columns-preview-short = 显示超出限制的行的预览。

flag-max-columns-preview-long = 为超过配置的最大列限制的行打印预览。
.sp
当使用 \flag{max-columns} 标志时，ripgrep 默认会完全替换任何太长的行，代之以一条指示匹配行已被移除的消息。当此标志与 \flag{max-columns} 结合使用时，将改为显示该行的预览（对应于限制大小），超过限制的行部分不显示。
.sp
如果未设置 \flag{max-columns} 标志，那么这没有任何效果。

flag-max-count-short = 限制匹配行的数量。

flag-max-count-long = 将每个被搜索文件的匹配行数限制为 \fINUM\fP。
.sp
当使用 \flag{multiline} 时，跨越多个行的单个匹配在此限制的目的下只计算一次。单行中的多个匹配只计算一次，就像在非多行模式中一样。
.sp
当与 \flag{after-context} 或 \flag{context} 结合使用时，如果上下文行包含匹配，则打印的匹配数可能会超过最大值。
.sp
请注意，\fB0\fP 是合法值，但可能没什么用。使用它时，ripgrep 不会搜索任何内容。

flag-max-depth-short = 最多向下递归 NUM 层目录。

flag-max-depth-long = 此标志将目录遍历的深度限制在给定路径之外 \fINUM\fP 层。\fB0\fP 值只搜索显式给定的路径本身。
.sp
例如，\fBrg --max-depth 0 \fP\fIdir/\fP 是无效操作，因为不会深入 \fIdir/\fP。\fBrg --max-depth 1 \fP\fIdir/\fP 将只搜索 \fIdir\fP 的直接子项。
.sp
此标志的另一种拼写是 \fB\-\-maxdepth\fP。

flag-max-filesize-short = 忽略大小超过 NUM 的文件。

flag-max-filesize-long = 忽略大小超过 \fINUM\fP 的文件。这不适用于目录。
.sp
输入格式接受 \fBK\fP、\fBM\fP 或 \fBG\fP 后缀，分别对应千字节、兆字节和千兆字节。如果没有提供后缀，则输入按字节处理。
.sp
示例：\fB\-\-max-filesize 50K\fP 或 \fB\-\-max\-filesize 80M\fP。

flag-mmap-short = 在可能时使用内存映射进行搜索。

flag-mmap-long = 启用后，ripgrep 将在可能时使用内存映射进行搜索。当 ripgrep 认为内存映射会更快时，默认启用。
.sp
内存映射搜索不能在所有情况下使用。例如，搜索虚拟文件或像 \fBstdin\fP 这样的流时。在这种情况下，即使启用了此标志，也不会使用内存映射。
.sp
请注意，如果 ripgrep 搜索的文件同时被截断，那么使用内存映射时 ripgrep 可能会意外中止。用户可以通过禁用内存映射来避免这种可能性。

flag-multiline-short = 启用跨多行搜索。

flag-multiline-long = 此标志启用跨多行搜索。
.sp
启用多行模式时，ripgrep 将解除匹配不能包含行终止符的限制。例如，当未启用多行模式（默认）时，正则表达式 \fB\\p{any}\fP 将匹配除 \fB\\n\fP 之外的任何 Unicode 码点。同样，正则表达式 \fB\\n\fP 被明确禁止，如果您尝试使用它，ripgrep 将返回错误。但是，启用多行模式时，\fB\\p{any}\fP 将匹配任何 Unicode 码点（包括 \fB\\n\fP），并且允许像 \fB\\n\fP 这样的正则表达式。
.sp
一个重要的注意事项是，多行模式不会改变 \fB.\fP 的匹配语义。也就是说，在大多数正则匹配器中，\fB.\fP 默认匹配除 \fB\\n\fP 之外的任何字符，ripgrep 也是如此。要使 \fB.\fP 匹配 \fB\\n\fP，您必须在正则表达式内启用"dot all"标志。例如，\fB(?s).\fP 和 \fB(?s:.)\fP 具有相同的语义，其中 \fB.\fP 将匹配任何字符，包括 \fB\\n\fP。或者，可以传递 \flag{multiline-dotall} 标志使"dot all"行为成为默认。此标志仅在启用多行搜索时适用。
.sp
单个匹配可以跨越的行数没有限制。
.sp
\fBWARNING\fP：由于底层正则引擎的工作方式，多行搜索可能比普通的面向行的搜索慢，并且可能使用更多内存。特别是，启用多行模式时，ripgrep 要求它搜索的每个文件在内存中连续布局（通过将其读入堆或通过内存映射）。无法内存映射的内容（如 \fBstdin\fP）将在搜索开始前一直消耗到 EOF。一般来说，ripgrep 只在必要时才做这些事情。具体来说，如果提供了 \flag{multiline} 标志，但正则表达式不包含会匹配 \fB\\n\fP 字符的模式，那么 ripgrep 将自动避免在搜索之前将每个文件读入内存。尽管如此，如果您只关心最多跨越一行的匹配，那么总是最好禁用多行模式。
.sp
这会覆盖 \flag{stop-on-nonmatch} 标志。

flag-multiline-dotall-short = 让 '.' 匹配行终止符。

flag-multiline-dotall-long = 此标志在所有正则模式中启用"dot all"模式。这使 \fB.\fP 在启用多行搜索时匹配行终止符。如果未使用 \flag{multiline} 标志启用多行搜索，此标志没有任何效果。
.sp
通常，\fB.\fP 匹配除行终止符之外的任何字符。虽然这种行为通常与面向行的匹配无关（因为匹配最多可以跨越一行），但在使用 \flag{multiline} 标志搜索时这可能很有用。默认情况下，多行模式在未启用"dot all"模式的情况下运行。
.sp
此标志通常用于别名或您的 ripgrep 配置文件中，如果您默认更喜欢"dot all"语义的话。请注意，无论是否使用此标志，"dot all"语义仍然可以通过正则模式本身中的内联标志来控制，例如，\fB(?s:.)\fP 总是启用"dot all"，而 \fB(?-s:.)\fP 总是禁用"dot all"。此外，您可以使用像 \fB\\p{any}\fP 这样的字符类来匹配任何 Unicode 码点，无论是否启用"dot all"模式。

flag-no-config-short = 永不读取配置文件。

flag-no-config-long = 设置后，ripgrep 将永不读取配置文件。存在此标志时，ripgrep 不会尊重 \fBRIPGREP_CONFIG_PATH\fP 环境变量。
.sp
如果 ripgrep 将来增加在预定义位置自动读取配置文件的功能，那么此标志也将禁用该行为。

flag-no-ignore-short = 不使用忽略文件。

flag-no-ignore-long = 设置后，诸如 \fB.gitignore\fP、\fB.ignore\fP 和 \fB.rgignore\fP 之类的忽略文件将不被尊重。这隐含 \flag{no-ignore-dot}、\flag{no-ignore-exclude}、\flag{no-ignore-global}、\flag{no-ignore-parent} 和 \flag{no-ignore-vcs}。
.sp
这不隐含 \flag{no-ignore-files}，因为 \flag{ignore-file} 是作为命令行参数显式指定的。
.sp
当只给出一次时，\flag{unrestricted} 标志的行为与此标志相同，可以被视为别名。但是，后续的 \flag{unrestricted} 标志有额外的效果。

flag-no-ignore-dot-short = 不使用 .ignore 或 .rgignore 文件。

flag-no-ignore-dot-long = 不尊重来自 \fB.ignore\fP 或 \fB.rgignore\fP 文件的过滤规则。
.sp
这不影响 ripgrep 是否会忽略名称以点开头的文件和目录。为此，请参阅 \flag{hidden} 标志。此标志也不影响是否尊重来自 \fB.gitignore\fP 文件的过滤规则。

flag-no-ignore-exclude-short = 不使用本地排除文件。

flag-no-ignore-exclude-long = 不尊重来自为仓库手动配置的文件的过滤规则。例如，这包括 \fBgit\fP 的 \fB.git/info/exclude\fP。

flag-no-ignore-files-short = 不使用 --ignore-file 参数。

flag-no-ignore-files-long = 设置后，任何 \flag{ignore-file} 标志（即使是在此标志之后给出的）都会被忽略。

flag-no-ignore-global-short = 不使用全局忽略文件。

flag-no-ignore-global-long = 不尊重来自"全局"来源的忽略文件的过滤规则，例如 \fBgit\fP 的 \fBcore.excludesFile\fP 配置选项（默认是 \fB$HOME/.config/git/ignore\fP）。

flag-no-ignore-messages-short = 抑制 gitignore 解析错误消息。

flag-no-ignore-messages-long = 启用此标志后，与解析忽略文件相关的所有错误消息都会被抑制。默认情况下，错误消息会打印到 stderr。在这些错误属于预期情况的场景中，可以使用此标志来避免看到消息产生的噪音。

flag-no-ignore-parent-short = 不使用父目录中的忽略文件。

flag-no-ignore-parent-long = 设置此标志后，不尊重来自父目录中忽略文件的过滤规则。默认情况下，ripgrep 会向上遍历当前工作目录的父目录，查找任何应该应用的忽略文件。在某些情况下这可能不是所期望的。

flag-no-ignore-vcs-short = 不使用来自版本控制的忽略文件。

flag-no-ignore-vcs-long = 给出后，不尊重来自版本控制忽略文件（例如 \fB.gitignore\fP）的过滤规则。默认情况下，ripgrep 尊重 \fBgit\fP 的忽略规则进行自动过滤。在某些情况下，可能不希望尊重版本控制的忽略规则，而只尊重 \fB.ignore\fP 或 \fB.rgignore\fP 中的规则。
.sp
请注意，此标志不会直接影响以点（\fB.\fP）开头的版本控制文件或文件夹（如 \fB.git\fP）的过滤。这些受 \flag{hidden} 及其相关标志的影响。
.sp
此标志对版本控制忽略文件也隐含 \flag{no-ignore-parent}。

flag-no-messages-short = 抑制某些错误消息。

flag-no-messages-long = 此标志抑制某些错误消息。具体来说，与打开和读取文件失败相关的消息。与模式语法相关的错误消息仍然会显示。

flag-no-pcre2unicode-short = （已弃用）为 PCRE2 禁用 Unicode 模式。

flag-no-pcre2unicode-long = 已弃用。请改用 \flag{no-unicode}。
.sp
请注意，Unicode 模式默认是启用的。

flag-no-require-git-short = 在 git 仓库之外也使用 .gitignore。

flag-no-require-git-long = 给出此标志时，即使不存在 \fBgit\fP 仓库，也会尊重诸如 \fB.gitignore\fP 之类的版本控制忽略文件。
.sp
默认情况下，只有当 ripgrep 检测到搜索是在版本控制仓库内执行时（例如，观察到 \fB.git\fP 目录），ripgrep 才会尊重来自版本控制忽略文件的过滤规则。
.sp
此标志放宽了默认限制。例如，当 \fBgit\fP 仓库的内容被存储或复制到某处，但仓库状态缺失时，它可能很有用。

flag-no-unicode-short = 禁用 Unicode 模式。

flag-no-unicode-long = 此标志为给予 ripgrep 的所有模式禁用 Unicode 模式。
.sp
默认情况下，ripgrep 会在其所有正则表达式中启用"Unicode 模式"。这有许多后果：
.sp
.IP \(bu 3n
\fB.\fP 将只匹配有效的 UTF-8 编码的 Unicode 标量值。
.sp
.IP \(bu 3n
像 \fB\\w\fP、\fB\\s\fP、\fB\\d\fP 这样的类都是 Unicode 感知的，并且比它们仅 ASCII 的版本大得多。
.sp
.IP \(bu 3n
不区分大小写的匹配将使用 Unicode 大小写折叠。
.sp
.IP \(bu 3n
大量像 \fB\\p{Emoji}\fP 这样的类可用。（虽然可用的具体类集因正则引擎而异。一般来说，默认正则引擎有更多可用的类。）
.sp
.IP \(bu 3n
词边界（\fB\\b\fP 和 \fB\\B\fP）使用 Unicode 对词字符的定义。
.PP
在某些情况下，关闭这些功能可能是可取的。此标志将完全做到这一点。例如，Unicode 模式有时会对性能产生负面影响，尤其是当像 \fB\\w\fP 这样的东西被频繁使用时（包括通过像 \fB\\w{100}\fP 这样的有界重复），而只需要它们的 ASCII 解释时。

flag-null-short = 在文件路径后打印 NUL 字节。

flag-null-long = 每当打印文件路径时，后面跟一个 \fBNUL\fP 字节。这包括在匹配之前打印文件路径，以及打印匹配文件列表时（例如使用 \flag{count}、\flag{files-with-matches} 和 \flag{files}）。此选项与 \fBxargs\fP 一起使用很有用。

flag-null-data-short = 使用 NUL 作为行终止符。

flag-null-data-long = 启用此标志会使 ripgrep 使用 \fBNUL\fP 作为行终止符，而不是默认的 \fP\\n\fP。
.sp
这在搜索大型二进制文件时很有用，如果使用 \fB\\n\fP 作为行终止符，这些文件会有非常长的行。特别是，ripgrep 要求每行至少必须能放进内存。改用 \fBNUL\fP 可能是保持低内存需求并避免 OOM（内存不足）状况的有用权宜之计。
.sp
这对于处理 NUL 分隔的数据也很有用，例如使用 ripgrep 的 \flag{null} 标志或 \fBfind\fP 的 \fB\-\-print0\fP 标志发出的数据。
.sp
使用此标志隐含 \flag{text}。它也覆盖 \flag{crlf}。

flag-one-file-system-short = 跳过其他文件系统上的目录。

flag-one-file-system-long = 启用后，ripgrep 不会相对于搜索开始的位置跨越文件系统边界。
.sp
请注意，这适用于给予 ripgrep 的每个路径参数。例如，在命令
.sp
.EX
    rg \-\-one\-file\-system /foo/bar /quux/baz
.EE
.sp
中，即使 \fI/foo/bar\fP 和 \fI/quux/baz\fP 位于不同的文件系统上，ripgrep 也会搜索两者，但在遍历每个路径的目录树时不会跨越文件系统边界。
.sp
这类似于 \fBfind\fP 的 \fB\-xdev\fP 或 \fB\-mount\fP 标志。

flag-only-matching-short = 只打印一行中匹配的部分。

flag-only-matching-long = 只打印匹配行中匹配的（非空）部分，每个这样的部分打印在单独的输出行上。

flag-path-separator-short = 设置打印路径时使用的路径分隔符。

flag-path-separator-long = 设置打印文件路径时使用的路径分隔符。这默认为您平台的分隔符，在 Unix 上是 \fB/\fP，在 Windows 上是 \fB\\\fP。此标志用于在环境要求时覆盖默认值（例如 cygwin）。路径分隔符限于单个字节。
.sp
将此标志设置为空字符串会将其恢复为默认行为。也就是说，路径分隔符会根据环境自动选择。

flag-passthru-short = 同时打印匹配和不匹配的行。

flag-passthru-long = 同时打印匹配和不匹配的行。
.sp
实现类似效果的另一种方式是修改您的模式以匹配空字符串。例如，如果您使用 \fBrg\fP \fIfoo\fP 搜索，那么改用 \fBrg\fP \fB'^|\fP\fIfoo\fP\fB'\fP 将输出每个被搜索文件中的每一行，但只有 \fIfoo\fP 的出现会被高亮。此标志无需修改模式即可启用相同的行为。
.sp
此标志的另一种拼写是 \fB\-\-passthrough\fP。
.sp
这会覆盖 \flag{context}、\flag{after-context} 和 \flag{before-context} 标志。

flag-pcre2-short = 启用 PCRE2 匹配。

flag-pcre2-long = 存在此标志时，ripgrep 将使用 PCRE2 正则引擎，而不是其默认正则引擎。
.sp
当您想使用环视或反向引用等特性时，这通常很有用。
.sp
使用此标志与传递 \fB\-\-engine=pcre2\fP 相同。用户也可以选择使用 \fB\-\-engine=auto\fP 让 ripgrep 根据给定的模式自动选择正确的正则引擎。此标志和 \flag{engine} 标志相互覆盖。
.sp
请注意，PCRE2 是 ripgrep 的可选特性。如果您的 ripgrep 构建中没有包含 PCRE2，那么使用此标志将导致 ripgrep 打印错误消息并退出。PCRE2 在某些情况下也可能有较差的用户体验，因为它的内省 API 比 ripgrep 的默认正则引擎少。例如，如果您在没有 \flag{multiline} 标志的情况下在 PCRE2 正则表达式中使用 \fB\\n\fP，那么 ripgrep 将静默地无法匹配任何内容，而不是立即报告错误（就像默认正则引擎那样）。

flag-pcre2version-short = 打印 ripgrep 使用的 PCRE2 版本。

flag-pcre2version-long = 存在此标志时，ripgrep 将打印正在使用的 PCRE2 版本以及其他信息，然后退出。如果 PCRE2 不可用，那么 ripgrep 将打印错误消息并以错误代码退出。

flag-pre-short = 搜索每个 PATH 上 COMMAND 的输出。

flag-pre-long = 对于每个输入 \fIPATH\fP，此标志使 ripgrep 搜索 \fICOMMAND\fP \fIPATH\fP 的标准输出，而不是 \fIPATH\fP 的内容。此选项期望 \fICOMMAND\fP 程序是路径或在您的 \fBPATH\fP 中可用。空字符串 \fICOMMAND\fP 或 \fB\-\-no\-pre\fP 标志都将禁用此行为。
.sp
.TP 12
\fBWARNING\fP
设置此标志时，ripgrep 将无条件地为每个被搜索的文件生成一个进程。因此，如果您本来不需要此标志提供的灵活性，这可能会造成不必要的巨大性能损失。一种可能的缓解措施是使用 \flag{pre-glob} 标志来限制预处理程序在哪些文件上运行。
.PP
当 ripgrep 搜索 stdin 时，不会运行预处理程序。
.sp
当搜索可能需要多种预处理程序之一的文件集时，\fICOMMAND\fP 应该是一个包装程序，它首先根据魔数/内容或 \fIPATH\fP 名称对 \fIPATH\fP 进行分类，然后分派到适当的预处理程序。为方便起见，每个 \fICOMMAND\fP 的标准输入也连接到 \fIPATH\fP。
.sp
例如，\fICOMMAND\fP 的 shell 脚本可能如下所示：
.sp
.EX
    case "$1" in
    *.pdf)
        exec pdftotext "$1" -
        ;;
    *)
        case $(file "$1") in
        *Zstandard*)
            exec pzstd -cdq
            ;;
        *)
            exec cat
            ;;
        esac
        ;;
    esac
.EE
.sp
上面的脚本使用 \fBpdftotext\fP 将 PDF 文件转换为纯文本。对于所有其他文件，脚本使用 \fBfile\fP 工具根据内容嗅探文件的类型。如果它是 Zstandard 格式的压缩文件，那么使用 \fBpzstd\fP 将内容解压缩到 stdout。
.sp
这会覆盖 \flag{search-zip} 标志。

flag-pre-glob-short = 从预处理程序中包含或排除文件。

flag-pre-glob-long = 此标志与 \flag{pre} 标志配合使用。也就是说，当给出一个或多个 \flag{pre-glob} 标志时，只有匹配给定 glob 集合的文件才会被交给 \flag{pre} 标志指定的命令。任何不匹配的文件将不使用预处理命令进行搜索。
.sp
此标志在使用 \flag{pre} 标志搜索许多文件时很有用。也就是说，它提供了避免为不需要预处理的文件产生进程开销的能力。例如，给定以下 shell 脚本 \fIpre-pdftotext\fP：
.sp
.EX
    #!/bin/sh
    pdftotext "$1" -
.EE
.sp
那么可以使用 \fB\-\-pre\fP \fIpre-pdftotext\fP \fB\-\-pre\-glob\fP '\fI*.pdf\fP' 使 ripgrep 只对具有 \fI.pdf\fP 扩展名的文件执行 \fIpre-pdftotext\fP 命令。
.sp
可以使用多个 \flag{pre-glob} 标志。glob 规则匹配 \fBgitignore\fP glob。在 glob 前加上 \fB!\fP 以排除它。
.sp
如果未使用 \flag{pre} 标志，此标志没有任何效果。

flag-pretty-short = colors、headings 和 line numbers 的别名。

flag-pretty-long = 这是 \fB\-\-color=always \-\-heading \-\-line\-number\fP 的便捷别名。当您即使将 ripgrep 的输出通过管道传给另一个程序或文件，仍希望获得漂亮的输出时，此标志很有用。例如：\fBrg -p \fP\fIfoo\fP \fB| less -R\fP。

flag-quiet-short = 不向 stdout 打印任何内容。

flag-quiet-long = 不向 stdout 打印任何内容。如果在文件中找到匹配，那么 ripgrep 将停止搜索。当 ripgrep 仅用于其退出代码时（如果没有找到匹配，退出代码将是错误代码），这很有用。
.sp
当使用 \flag{files} 时，ripgrep 在找到第一个不匹配任何忽略规则的文件后将停止查找文件。

flag-regex-size-limit-short = 编译后正则表达式的大小限制。

flag-regex-size-limit-long = 编译后正则表达式的大小限制，其中编译后的正则表达式通常对应于内存中的单个对象，可以匹配提供给 ripgrep 的所有模式。默认限制足够宽裕，大多数合理的模式（甚至少量的模式）都应该能装下。
.sp
当您明确想让 ripgrep 花费可能更多的时间和/或内存来构建正则匹配器时，更改此值很有用。
.sp
输入格式接受 \fBK\fP、\fBM\fP 或 \fBG\fP 后缀，分别对应千字节、兆字节和千兆字节。如果没有提供后缀，则输入按字节处理。

flag-regexp-short = 要搜索的模式。

flag-regexp-long = 要搜索的模式。此选项可以多次提供，此时除了 \flag{file} 提供的任何模式外，还会搜索所有给定的模式。匹配至少一个所提供模式的行会被打印。当搜索以破折号开头的模式时，也可以使用此标志。
.sp
例如，要搜索字面量 \fB\-foo\fP：
.sp
.EX
    rg \-e \-foo
.EE
.sp
您也可以使用特殊的 \fB\-\-\fP 分隔符来表示不再提供任何标志。也就是说，以下内容等同于上面：
.sp
.EX
    rg \-\- \-foo
.EE
.sp
当使用 \flag{file} 或 \flag{regexp} 时，ripgrep 将所有位置参数视为要搜索的文件或目录。

flag-replace-short = 用给定文本替换匹配。

flag-replace-long = 在打印结果时用给定文本替换每个匹配。此标志和任何其他 ripgrep 标志都不会修改您的文件。
.sp
替换字符串中支持捕获组索引（例如 \fB$\fP\fI5\fP）和名称（例如 \fB$\fP\fIfoo\fP）。捕获组索引根据组左括号的位置编号，其中最左边的组是 \fB$\fP\fI1\fP。特殊的 \fB$\fP\fI0\fP 组对应于整个匹配。
.sp
组的名称由 \fB$\fP 之后最长的字母、数字和下划线字符串（即 \fB[_0-9A-Za-z]\fP）组成。例如，\fB$\fP\fI1a\fP 将被替换为名为 \fI1a\fP 的组，而不是索引为 \fI1\fP 的组。如果组的名称包含不是字母、数字或下划线的字符，或者您想立即在组后面跟另一个字符串，则名称应放在大括号内。例如，\fB${\fP\fI1\fP\fB}\fP\fIa\fP 将获取索引为 \fI1\fP 的组的内容并将 \fIa\fP 追加到其末尾。
.sp
如果索引或名称不引用有效的捕获组，它将被替换为空字符串。
.sp
在 Bash 和 zsh 等 shell 中，您应该用单引号而不是双引号包裹模式。否则，捕获组索引将被展开的 shell 变量替换，而这些变量很可能为空。
.sp
要写入字面量 \fB$\fP，请使用 \fB$$\fP。
.sp
请注意，默认情况下替换替换每个匹配，而不是整行。要替换整行，您应该匹配整行。
.sp
此标志可以与 \flag{only-matching} 标志一起使用。

flag-search-zip-short = 在压缩文件中搜索。

flag-search-zip-long = 此标志指示 ripgrep 在压缩文件中搜索。目前支持 gzip、bzip2、xz、LZ4、LZMA、Brotli 和 Zstd 文件。此选项期望解压缩二进制文件（如 \fBgzip\fP）在您的 \fBPATH\fP 中可用。如果找不到所需的二进制文件，那么默认情况下 ripgrep 不会发出错误消息。使用 \flag{debug} 标志查看更多信息。
.sp
请注意，此标志不会使 ripgrep 将存档格式作为目录树搜索。它只会使 ripgrep 检测压缩文件，然后在搜索其内容之前解压缩它们，就像处理任何其他文件一样。
.sp
这会覆盖 \flag{pre} 标志。

flag-smart-case-short = 智能大小写搜索。

flag-smart-case-long = 此标志指示 ripgrep 在模式全为小写时不区分大小写地搜索。否则，ripgrep 将区分大小写地搜索。
.sp
如果以下两个规则都成立，则认为模式全为小写：
.sp
.IP \(bu 3n
首先，模式包含至少一个字面量字符。例如，\fBa\\w\fP 包含一个字面量（\fBa\fP），但仅 \fB\\w\fP 不包含。
.sp
.IP \(bu 3n
其次，在模式中的字面量中，根据 Unicode，没有一个是大小写被视为大写的。例如，\fBfoo\\pL\fP 没有大写字面量，但 \fBFoo\\pL\fP 有。
.PP
这会覆盖 \flag{case-sensitive} 和 \flag{ignore-case} 标志。

flag-sort-files-short = （已弃用）按文件路径对结果排序。

flag-sort-files-long = 已弃用。请改用 \fB\-\-sort=path\fP。
.sp
此标志指示 ripgrep 按文件路径的字典序升序对搜索结果排序。请注意，这目前会禁用所有并行性，并在单线程中运行搜索。
.sp
此标志覆盖 \flag{sort} 和 \flag{sortr}。

flag-sort-short = 按升序对结果排序。

flag-sort-long = 此标志启用结果的升序排序。此标志的可能值为：
.sp
.TP 12
\fBnone\fP
（默认）不对结果排序。最快。可以多线程。
.TP 12
\fBpath\fP
按文件路径排序。始终单线程。顺序由遍历期间对每个目录条目中的文件排序决定。这意味着给定文件 \fBa/b\fP 和 \fBa+\fP，后者将排在前者之后，即使 \fB+\fP 通常会排在 \fB/\fP 之前。
.TP 12
\fBmodified\fP
按文件的最后修改时间排序。始终单线程。
.TP 12
\fBaccessed\fP
按文件的最后访问时间排序。始终单线程。
.TP 12
\fBcreated\fP
按文件的创建时间排序。始终单线程。
.PP
如果所选（手动或默认）的排序标准在您的系统上不可用（例如，ext4 文件系统上创建时间不可用），那么 ripgrep 将尝试检测这一点，打印错误并在不搜索的情况下退出。
.sp
要以相反或降序对结果排序，请使用 \flag{sortr} 标志。此外，此标志覆盖 \flag{sortr}。
.sp
请注意，对结果排序目前总是迫使 ripgrep 放弃并行性并在单线程中运行。

flag-sortr-short = 按降序对结果排序。

flag-sortr-long = 此标志启用结果的降序排序。此标志的可能值为：
.sp
.TP 12
\fBnone\fP
（默认）不对结果排序。最快。可以多线程。
.TP 12
\fBpath\fP
按文件路径排序。始终单线程。顺序由遍历期间对每个目录条目中的文件排序决定。这意味着给定文件 \fBa/b\fP 和 \fBa+\fP，后者将排在前者之前，即使在进行反向字典序排序时 \fB+\fP 通常会排在 \fB/\fP 之后。
.TP 12
\fBmodified\fP
按文件的最后修改时间排序。始终单线程。
.TP 12
\fBaccessed\fP
按文件的最后访问时间排序。始终单线程。
.TP 12
\fBcreated\fP
按文件的创建时间排序。始终单线程。
.PP
如果所选（手动或默认）的排序标准在您的系统上不可用（例如，ext4 文件系统上创建时间不可用），那么 ripgrep 将尝试检测这一点，打印错误并在不搜索的情况下退出。
.sp
要以升序对结果排序，请使用 \flag{sort} 标志。此外，此标志覆盖 \flag{sort}。
.sp
请注意，对结果排序目前总是迫使 ripgrep 放弃并行性并在单线程中运行。

flag-stats-short = 打印关于搜索的统计信息。

flag-stats-long = 启用后，ripgrep 将打印关于搜索的汇总统计信息。存在此标志时，ripgrep 将在搜索结束时至少向 stdout 打印以下统计信息：匹配行数、包含匹配的文件数、被搜索的文件数以及整个搜索完成所花费的时间。
.sp
这组汇总统计信息可能会随时间扩展。
.sp
当使用 \flag{json} 时，此标志总是被隐式启用。
.sp
请注意，如果传递了 \flag{files}、\flag{files-with-matches} 或 \flag{files-without-match}，此标志没有任何效果。

flag-stop-on-nonmatch-short = 在出现不匹配后停止搜索。

flag-stop-on-nonmatch-long = 启用此选项将使 ripgrep 在遇到匹配行之后遇到不匹配行时停止读取文件。如果预期给定文件中的所有匹配都在连续行上（例如由于行已排序），这很有用。
.sp
这会覆盖 \flag{multiline} 标志。

flag-text-short = 将二进制文件当作文本搜索。

flag-text-long = 此标志指示 ripgrep 将二进制文件当作文本搜索。存在此标志时，ripgrep 的二进制文件检测被禁用。这意味着当搜索二进制文件时，如果存在匹配，其内容可能会被打印。这可能会导致打印改变终端行为的转义码。
.sp
启用二进制文件检测时，它并不完美。一般来说，它使用一个简单的启发式规则。如果在搜索过程中看到 \fBNUL\fP 字节，那么该文件被认为是二进制的，搜索停止（除非存在此标志）。或者，如果使用 \flag{binary} 标志，那么 ripgrep 只会在看到匹配之后（或搜索完整个文件之后）看到 \fBNUL\fP 字节时才退出。
.sp
此标志覆盖 \flag{binary} 标志。

flag-threads-short = 设置要使用的近似线程数。

flag-threads-long = 此标志设置要使用的近似线程数。\fB0\fP 值（这是默认值）使 ripgrep 使用启发式规则选择线程数。

flag-trace-short = 显示跟踪消息。

flag-trace-long = 显示跟踪消息。这比 \flag{debug} 标志显示更多细节。一般来说，只有在 \flag{debug} 没有发出您要查找的信息时才应该使用它。

flag-trim-short = 去除匹配的前缀空白。

flag-trim-long = 设置后，打印的每行开头的所有 ASCII 空白都会被移除。

flag-type-short = 只搜索匹配 TYPE 的文件。

flag-type-long = 此标志将 ripgrep 限制为搜索匹配 \fITYPE\fP 的文件。可以提供多个 \flag{type} 标志。
.sp
此标志支持特殊值 \fBall\fP，它的行为就像为 ripgrep 支持的每个文件类型（包括任何自定义文件类型）都提供了 \flag{type} 一样。最终结果是 \fB\-\-type=all\fP 使 ripgrep 以"白名单"模式搜索，它只搜索通过其类型定义识别的文件。
.sp
请注意，此标志的优先级低于 \flag{glob} 标志和忽略文件中的任何规则。
.sp
要查看可用文件类型的列表，请使用 \flag{type-list} 标志。

flag-type-add-short = 为文件类型添加新的 glob。

flag-type-add-long = 此标志为特定文件类型添加新的 glob。一次只能添加一个 glob。可以提供多个 \flag{type-add} 标志。除非使用 \flag{type-clear}，否则 glob 会添加到 ripgrep 内部定义的任何现有 glob 中。
.sp
请注意，每次调用 ripgrep 都必须传递此标志。类型设置不会持久化。有关解决方法，请参阅 \fBCONFIGURATION FILES\fP。
.sp
示例：
.sp
.EX
    rg \-\-type\-add 'foo:*.foo' -tfoo \fIPATTERN\fP
.EE
.sp
此标志也可以用于通过特殊的 include 指令从其他类型包含规则。include 指令允许指定一个或多个已定义的其他类型名称（用逗号分隔），其规则将自动导入到指定的类型中。例如，要创建名为 src 的匹配 C++、Python 和 Markdown 文件的类型，可以使用：
.sp
.EX
    \-\-type\-add 'src:include:cpp,py,md'
.EE
.sp
通过再次使用此标志，仍然可以向 src 类型添加额外的 glob 规则：
.sp
.EX
    \-\-type\-add 'src:include:cpp,py,md' \-\-type\-add 'src:*.foo'
.EE
.sp
请注意，类型名称必须只由 Unicode 字母或数字组成。不允许使用标点字符。

flag-type-clear-short = 清除文件类型的 glob。

flag-type-clear-long = 清除以前为 \fITYPE\fP 定义的文件类型 glob。这会清除为该 \fITYPE\fP 先前定义的所有 glob，但在此标志之后可以添加 glob。
.sp
请注意，每次调用 ripgrep 都必须传递此标志。类型设置不会持久化。有关解决方法，请参阅 \fBCONFIGURATION FILES\fP。

flag-type-not-short = 不搜索匹配 TYPE 的文件。

flag-type-not-long = 不搜索匹配 \fITYPE\fP 的文件。可以提供多个 \flag{type-not} 标志。使用 \flag{type-list} 标志列出所有可用的类型。
.sp
此标志支持特殊值 \fBall\fP，它的行为就像为 ripgrep 支持的每个文件类型（包括任何自定义文件类型）都提供了 \flag{type-not} 一样。最终结果是 \fB\-\-type\-not=all\fP 使 ripgrep 以"黑名单"模式搜索，它只搜索其类型定义无法识别的文件。
.sp
要查看可用文件类型的列表，请使用 \flag{type-list} 标志。

flag-type-list-short = 显示所有支持的文件类型。

flag-type-list-long = 显示所有支持的文件类型及其对应的 glob。这会考虑到任何给定的 \flag{type-add} 和 \flag{type-clear} 标志。每种类型单独打印在一行上，后面跟一个 \fB:\fP，然后是同一行上该类型的逗号分隔的 glob 列表。

flag-unrestricted-short = 降低“智能”过滤的级别。

flag-unrestricted-long = 此标志降低"智能"过滤的级别。重复使用（最多 3 次）会进一步降低过滤。重复三次时，ripgrep 将搜索目录树中的每个文件。
.sp
单个 \flag{unrestricted} 标志等同于 \flag{no-ignore}。两个 \flag{unrestricted} 标志等同于 \flag{no-ignore} \flag{hidden}。三个 \flag{unrestricted} 标志等同于 \flag{no-ignore} \flag{hidden} \flag{binary}。
.sp
给出 \fB-uuu\fP 时，ripgrep 仍然会做的唯一过滤是跳过符号链接和避免打印二进制文件中的匹配。符号链接可以通过 \flag{follow} 标志跟随，二进制文件可以通过 \flag{text} 标志当作文本文件处理。

flag-version-short = 打印 ripgrep 的版本。

flag-version-long = 此标志打印 ripgrep 的版本。这也可能打印其他相关信息，例如是否存在目标特定的优化，以及此 ripgrep 构建所编译自的 \fBgit\fP 修订版本。

flag-vimgrep-short = 以 vim 兼容格式打印结果。

flag-vimgrep-long = 此标志指示 ripgrep 打印结果时让每个匹配占一行，包括行号和列号。
.sp
使用此选项时，一行有多个匹配的行会被整体打印多次。因此，此标志导致的输出总量可能是输入大小的二次方。例如，如果模式匹配输入文件中的每个字节，那么每一行都会为每个匹配的字节重复。因此，用户只应在别无选择时使用此标志。编辑器集成应该更喜欢其他从 ripgrep 读取结果的方式，例如通过 \flag{json} 标志。避免过度内存使用的一种替代方案是使用 \flag{threads} 标志强制 ripgrep 进入单线程模式。不过请注意，这不会影响输出的总大小，只会影响 ripgrep 将使用的堆内存。

flag-with-filename-short = 在每行匹配后打印文件路径。

flag-with-filename-long = 此标志指示 ripgrep 为每个匹配行打印文件路径。当搜索多个文件时，这是默认行为。如果启用了 \flag{heading}（打印到 tty 时的默认），文件路径将显示在每个文件的匹配簇上方；否则，文件名将作为每个匹配行的前缀显示。
.sp
此标志覆盖 \flag{no-filename}。

flag-with-filename-no-short = 永不打印每行匹配的路径。

flag-with-filename-no-long = 此标志指示 ripgrep 永不打印每个匹配行的文件路径。当 ripgrep 被显式指示搜索一个文件或 stdin 时，这是默认行为。
.sp
此标志覆盖 \flag{with-filename}。

flag-word-regexp-short = 显示以词边界包围的匹配。

flag-word-regexp-long = 启用后，ripgrep 将只显示以词边界包围的匹配。这等同于将每个模式用 \fB\\b{start-half}\fP 和 \fB\\b{end-half}\fP 包围。这些是 ripgrep 默认正则引擎的自定义语法，与 \fB\\b\fP 不同，它不要求在某一侧匹配一个词字符。也就是说，\fB\\b{start-half}\fP 对应于在左侧匹配 \fB\\W|\\A\fP，\fB\\b{end-half}\fP 对应于在右侧匹配 \fB\\W|\\z\fP。
.sp
这会覆盖 \flag{line-regexp} 标志。

template-help-short = ripgrep !!VERSION!!
Andrew Gallant <jamslam@gmail.com>

ripgrep (rg) 递归搜索当前目录中匹配正则模式的行。默认情况下，ripgrep 会尊重 gitignore 规则，并自动跳过隐藏文件/目录和二进制文件。

使用 -h 查看简短描述，使用 --help 查看更多详细信息。

项目主页：https://github.com/BurntSushi/ripgrep

用法：
  rg [选项] 模式 [路径 ...]

位置参数：
  <模式>   用于搜索的正则表达式。
  <路径>...   要搜索的文件或目录。

输入选项：
!!input!!

搜索选项：
!!search!!

过滤选项：
!!filter!!

输出选项：
!!output!!

输出模式：
!!output-modes!!

索引：
!!indexing!!

日志选项：
!!logging!!

其他行为：
!!other-behaviors!!

template-help-long = ripgrep !!VERSION!!
Andrew Gallant <jamslam@gmail.com>

ripgrep (rg) 递归搜索当前目录中匹配正则模式的行。默认情况下，ripgrep 会尊重 gitignore 规则，并自动跳过隐藏文件/目录和二进制文件。

使用 -h 查看简短描述，使用 --help 查看更多详细信息。

项目主页：https://github.com/BurntSushi/ripgrep

用法：
    rg [选项] 模式 [路径 ...]
    rg [选项] -e 模式 ... [路径 ...]
    rg [选项] -f 模式文件 ... [路径 ...]
    rg [选项] --files [路径 ...]
    rg [选项] --type-list
    command | rg [选项] 模式
    rg [选项] --help
    rg [选项] --version

位置参数：
    <模式>
        用于搜索的正则表达式。要匹配以破折号开头的模式，请使用 -e/--regexp 标志。

        例如，要搜索字面量 '-foo'，可以使用此标志：

            rg -e -foo

        您也可以使用特殊的 '--' 分隔符来表示不再提供任何标志。也就是说，以下内容等同于上面：

            rg -- -foo

    <路径>...
        要搜索的文件或目录。目录会递归搜索。命令行上指定的文件路径会覆盖 glob 和忽略规则。

输入选项：
!!input!!

搜索选项：
!!search!!

过滤选项：
!!filter!!

输出选项：
!!output!!

输出模式：
!!output-modes!!

索引：
!!indexing!!

日志选项：
!!logging!!

其他行为：
!!other-behaviors!!

template-man = .TH RG 1 2026-07-15 "!!VERSION!!" "用户命令"
.
.
.SH 名称
rg \- 递归搜索当前目录中匹配模式的行
.
.
.SH 概要
.\" I considered using GNU troff's .SY and .YS "synopsis" macros here, but it
.\" looks like they aren't portable. Specifically, they don't appear to be in
.\" BSD's mdoc used on macOS.
.sp
\fBrg\fP [\fI选项\fP] \fI模式\fP [\fI路径\fP...]
.sp
\fBrg\fP [\fI选项\fP] \fB\-e\fP \fI模式\fP... [\fI路径\fP...]
.sp
\fBrg\fP [\fI选项\fP] \fB\-f\fP \fI模式文件\fP... [\fI路径\fP...]
.sp
\fBrg\fP [\fI选项\fP] \fB\-\-files\fP [\fI路径\fP...]
.sp
\fBrg\fP [\fI选项\fP] \fB\-\-type\-list\fP
.sp
\fIcommand\fP | \fBrg\fP [\fI选项\fP] \fI模式\fP
.sp
\fBrg\fP [\fI选项\fP] \fB\-\-help\fP
.sp
\fBrg\fP [\fI选项\fP] \fB\-\-version\fP
.
.
.SH 描述
ripgrep (rg) 递归搜索当前目录中匹配正则模式的行。默认情况下，ripgrep 会尊重您的 \fB.gitignore\fP，并自动跳过隐藏文件/目录和二进制文件。
.sp
ripgrep 的默认正则引擎使用有限自动机，并保证线性时间搜索。因此，不支持反向引用和任意环视等特性。但是，如果 ripgrep 使用 PCRE2 构建，那么可以使用 \fB\-P/\-\-pcre2\fP 标志来启用反向引用和环视。
.sp
ripgrep 支持配置文件。将 \fBRIPGREP_CONFIG_PATH\fP 设置为配置文件。该文件可以每行指定一个 shell 参数。以 \fB#\fP 开头的行会被忽略。有关更多详细信息，请参阅下面的 \fBCONFIGURATION FILES\fP（配置文件）。
.sp
ripgrep 会自动检测 stdin 是否是可读文件，并搜索 stdin 中的正则模式，例如 \fBls | rg foo\fP。在某些环境中，stdin 可能存在但不应该存在。要关闭 stdin 检测，可以显式指定要搜索的目录，例如 \fBrg foo ./\fP。
.sp
与其他工具（如 \fBls\fP）一样，ripgrep 会根据 stdout 是否连接到 tty 来改变其输出。默认情况下，打印到 tty 时，ripgrep 会启用颜色、行号以及标题格式，该格式将每个匹配文件路径列出一次，而不是每个匹配行列出一次。
.sp
提示：要禁用所有智能过滤并使 ripgrep 的行为更像经典的 grep，请使用 \fBrg -uuu\fP。
.
.
.SH 正则语法
ripgrep 默认使用 Rust 的正则引擎，其语法文档如下：
\fIhttps://docs.rs/regex/1.*/regex/#syntax\fP
.sp
ripgrep 使用面向字节的正则表达式，这有一些额外的文档：
\fIhttps://docs.rs/regex/1.*/regex/bytes/index.html#syntax\fP
.sp
粗略地说，ripgrep 使用类似 Perl 的正则表达式，但不支持环视或反向引用。这使它们与 *egrep* 支持的"扩展"（ERE）正则表达式非常相似，但具有一些额外的特性，如 Unicode 字符类。
.sp
如果您将 ripgrep 与 \fB\-P/\-\-pcre2\fP 标志一起使用，那么请查阅 \fIhttps://www.pcre.org\fP 或 PCRE2 手册页以获取所支持语法的文档。
.
.
.SH 位置参数
.TP 12
\fI模式\fP
用于搜索的正则表达式。要匹配以破折号开头的模式，请使用 \fB\-e/\-\-regexp\fP 选项。
.TP 12
\fI路径\fP
要搜索的文件或目录。目录会递归搜索。命令行上显式指定的文件路径会覆盖 glob 和忽略规则。
.
.
.SH 选项
本节记录 ripgrep 接受的所有标志。下面的标志根据其功能分组。
.sp
请注意，许多选项可以打开和关闭。在某些情况下，这些标志没有在下面明确列出。例如，\fB\-\-column\fP 标志（在下面列出）在 ripgrep 的输出中启用列号，但 \fB\-\-no\-column\fP 标志（未在下面列出）禁用它们。相反的情况也可能存在。例如，\fB\-\-no\-ignore\fP 标志（在下面列出）禁用 ripgrep 的 \fBgitignore\fP 逻辑，但 \fB\-\-ignore\fP 标志（未在下面列出）启用它。这些标志对于在命令行上覆盖 ripgrep 配置文件（或别名）很有用。每个标志的文档都会说明是否存在反转标志。在所有情况下，最后指定的标志优先。
.
.SS 输入选项
!!input!!
.
.SS 搜索选项
!!search!!
.
.SS 过滤选项
!!filter!!
.
.SS 输出选项
!!output!!
.
.SS 输出模式
!!output-modes!!
.
.SS 索引
!!indexing!!
.
.SS 日志选项
!!logging!!
.
.SS 其他行为
!!other-behaviors!!
.
.
.SH 退出状态
如果 ripgrep 找到匹配，那么程序的退出状态为 \fB0\fP。如果找不到匹配，那么退出状态为 \fB1\fP。如果发生错误，那么退出状态总是 \fB2\fP，除非 ripgrep 使用 \fB\-q/\-\-quiet\fP 标志运行且找到了匹配。总结如下：
.sp
.IP \(bu 3n
只有当至少找到一个匹配且没有发生错误时，才会出现 \fB0\fP 退出状态，除非给出了 \fB\-q/\-\-quiet\fP。
.
.IP \(bu 3n
只有当没有找到匹配且没有发生错误时，才会出现 \fB1\fP 退出状态。
.
.IP \(bu 3n
当发生错误时，会出现 \fB2\fP 退出状态。这既适用于灾难性错误（例如，正则表达式语法错误），也适用于软错误（例如，无法读取文件）。
.
.
.SH 自动过滤
ripgrep 默认会进行相当多的自动过滤。本节描述该过滤以及如何控制它。
.sp
\fBTIP\fP：要禁用自动过滤，请使用 \fBrg -uuu\fP。
.sp
ripgrep 自动的"智能"过滤是 ripgrep 与 \fBgrep\fP 等其他工具之间最明显的区别特性之一。因此，对于不期望它的用户来说，其行为可能令人惊讶。
.sp
ripgrep 自动执行四种类型的过滤：
.sp
.
.IP 1. 3n
匹配忽略规则的文件和目录不会被搜索。
.IP 2. 3n
隐藏文件和目录不会被搜索。
.IP 3. 3n
二进制文件（包含 \fBNUL\fP 字节的文件）不会被搜索。
.IP 4. 3n
符号链接不会被跟随。
.PP
第一种类型的过滤是最复杂的。ripgrep 将尽可能忠实地尊重您的 \fBgitignore\fP 规则。特别是，这包括以下内容：
.
.IP \(bu 3n
任何全局规则，例如 \fB$HOME/.config/git/ignore\fP 中的规则。
.
.IP \(bu 3n
相关 \fB.gitignore\fP 文件中的任何规则。这包括属于同一 \fBgit\fP 仓库的父目录中的 \fB.gitignore\fP 文件。（除非给出 \fB\-\-no\-require\-git\fP。）
.
.IP \(bu 3n
任何本地规则，例如 \fB.git/info/exclude\fP 中的规则。
.PP
在某些情况下，ripgrep 和 \fBgit\fP 在哪些文件被忽略方面不会总是同步。例如，通过 \fB.gitignore\fP 被忽略但被 \fBgit\fP 跟踪的文件不会被 ripgrep 搜索，即使 \fBgit\fP 跟踪它。这不太可能被修复。相反，您应该确保您的排除规则与您跟踪的文件精确匹配，或者使用 \fBgit grep\fP 进行搜索。
.sp
可以在 \fBgit\fP 上下文之外提供额外的忽略规则：
.
.IP \(bu 3n
\fB.ignore\fP 中的任何规则。ripgrep 也会尊重父目录中的 \fB.ignore\fP 文件。
.
.IP \(bu 3n
\fB.rgignore\fP 中的任何规则。ripgrep 也会尊重父目录中的 \fB.rgignore\fP 文件。
.
.IP \(bu 3n
使用 \fB\-\-ignore\-file\fP 标志指定的文件中的任何规则。
.PP
忽略规则的优先级如下，后面的项目覆盖前面的项目：
.
.IP \(bu 3n
由 \fB\-\-ignore\-file\fP 给出的文件。
.
.IP \(bu 3n
全局 gitignore 规则，例如来自 \fB$HOME/.config/git/ignore\fP 的规则。
.
.IP \(bu 3n
来自 \fB.git/info/exclude\fP 的本地规则。
.
.IP \(bu 3n
来自 \fB.gitignore\fP 的规则。
.
.IP \(bu 3n
来自 \fB.ignore\fP 的规则。
.
.IP \(bu 3n
来自 \fB.rgignore\fP 的规则。
.PP
例如，如果 \fIfoo\fP 在 \fB.gitignore\fP 中，而 \fB!\fP\fIfoo\fP 在 \fB.rgignore\fP 中，那么 \fIfoo\fP 不会被忽略，因为 \fB.rgignore\fP 优先于 \fB.gitignore\fP。
.sp
每种类型的过滤都可以通过命令行标志进行配置：
.
.IP \(bu 3n
有几个以 \fB\-\-no\-ignore\fP 开头的标志可以切换尊重哪些忽略规则（如果有的话）。\fB\-\-no\-ignore\fP 本身会禁用所有规则。
.
.IP \(bu 3n
\fB\-./\-\-hidden\fP 将强制 ripgrep 搜索隐藏文件和目录。
.
.IP \(bu 3n
\fB\-\-binary\fP 将强制 ripgrep 搜索二进制文件。
.
.IP \(bu 3n
\fB\-L/\-\-follow\fP 将强制 ripgrep 跟随符号链接。
.PP
作为特殊的简写，\fB\-u\fP 标志可以最多指定三次。每多指定一次都会逐步减少过滤：
.
.IP \(bu 3n
\fB\-u\fP 等同于 \fB\-\-no\-ignore\fP。
.
.IP \(bu 3n
\fB\-uu\fP 等同于 \fB\-\-no\-ignore \-\-hidden\fP。
.
.IP \(bu 3n
\fB\-uuu\fP 等同于 \fB\-\-no\-ignore \-\-hidden \-\-binary\fP。
.PP
特别是，\fBrg -uuu\fP 应该搜索与 \fBgrep -r\fP 完全相同的内容。
.
.
.SH 配置文件
ripgrep 支持读取改变 ripgrep 默认行为的配置文件。配置文件的格式是"rc"风格，非常简单。它由两条规则定义：
.
.IP 1. 3n
每一行都是一个 shell 参数（去除空白后）。
.
.IP 2. 3n
以 \fB#\fP 开头的行（前面可以有任意数量的空白）会被忽略。
.PP
当且仅当 \fBRIPGREP_CONFIG_PATH\fP 环境变量被设置且非空时，ripgrep 才会查找单个配置文件。ripgrep 会在启动时解析此文件中的参数，其行为就像此文件中的参数被前置到命令行上显式给出的任何参数之前一样。不过请注意，您运行的 \fBrg\fP 命令仍然必须是有效的。也就是说，即使在配置文件中使用了 \fB\-e/\-\-regexp\fP 标志，命令行上也必须始终包含至少一个模式。
.sp
例如，如果您的 ripgreprc 文件包含一行：
.sp
.EX
    \-\-smart\-case
.EE
.sp
那么以下命令
.sp
.EX
    RIPGREP_CONFIG_PATH=wherever/.ripgreprc rg foo
.EE
.sp
的行为将与以下命令完全相同：
.sp
.EX
    rg \-\-smart-case foo
.EE
.sp
另一个示例是添加类型，如下所示：
.sp
.EX
    \-\-type-add
    web:*.{html,css,js}*
.EE
.sp
上面的行为将与以下命令完全相同：
.sp
.EX
    rg \-\-type\-add 'web:*.{html,css,js}*' foo
.EE
.sp
使用 glob 也是如此。这个：
.sp
.EX
    \-\-glob=!.git
.EE
.sp
或者这个：
.sp
.EX
    \-\-glob
    !.git
.EE
.sp
的行为将与以下命令完全相同：
.sp
.EX
    rg \-\-glob '!.git' foo
.EE
.sp
底线是每个 shell 参数都需要单独占一行。例如，包含以下内容的配置文件
.sp
.EX
    \-j 4
.EE
.sp
可能不是您想要的效果。相反，您需要
.sp
.EX
    \-j
    4
.EE
.sp
或
.sp
.EX
    \-j4
.EE
.sp
ripgrep 还提供了一个标志 \fB\-\-no\-config\fP，当存在该标志时，将抑制对配置的任何和所有支持。这包括将来从预定义路径自动加载配置文件的任何支持。
.sp
配置文件与显式参数之间的冲突的处理方式与同一命令行调用中的冲突完全一样。也就是说，假设您的配置文件只包含 \fB\-\-smart\-case\fP，那么此命令：
.sp
.EX
    RIPGREP_CONFIG_PATH=wherever/.ripgreprc rg foo \-\-case\-sensitive
.EE
.sp
完全等同于
.sp
.EX
    rg \-\-smart\-case foo \-\-case\-sensitive
.EE
.sp
在这种情况下，\fB\-\-case\-sensitive\fP 标志将覆盖 \fB\-\-smart\-case\fP 标志。
.
.
.SH shell 补全
发布压缩包中包含 Bash、Fish、Zsh 和 PowerShell 的 shell 补全文件。
.sp
对于 \fBbash\fP，将 \fBrg.bash\fP 移动到 \fB$XDG_CONFIG_HOME/bash_completion\fP 或 \fB/etc/bash_completion.d/\fP。
.sp
对于 \fBfish\fP，将 \fBrg.fish\fP 移动到 \fB$HOME/.config/fish/completions\fP。
.sp
对于 \fBzsh\fP，将 \fB_rg\fP 移动到您的某个 \fB$fpath\fP 目录。
.
.
.SH 注意事项
如果 ripgrep 搜索的文件同时被截断，那么在默认设置下 ripgrep 可能会意外中止。可以通过传递 \fB\-\-no\-mmap\fP 标志来避免此行为，该标志将在所有情况下强制禁用内存映射的使用。
.sp
ripgrep 可能会根据几个因素使用大量内存。首先，如果 ripgrep 使用并行搜索（默认），那么每个单独文件的整个输出都会被缓冲到内存中，以防止输出中的匹配交错。要避免这种情况，可以使用 \fB\-j1\fP 标志禁用并行。其次，ripgrep 总是需要至少在内存中保留一行才能执行搜索。因此，一行非常长的文件可能导致 ripgrep 使用大量内存。一般来说，这只有在启用 \fB\-a/\-\-text\fP 标志搜索二进制数据时才会发生。（当未启用 \fB\-a/\-\-text\fP 标志时，ripgrep 会将所有 NUL 字节替换为行终止符，这通常可以防止过度的内存使用。）第三，当 ripgrep 使用内存映射搜索大文件时，进程可能会将其常驻内存使用量报告为文件大小。但是，这并不意味着 ripgrep 真的需要使用那么多堆内存；操作系统通常会为您处理这个问题。
.
.
.SH 版本
!!VERSION!!
.
.
.SH 主页
\fIhttps://github.com/BurntSushi/ripgrep\fP
.sp
请将错误和功能请求报告给问题跟踪器。请尽力为错误提供可重现的测试用例。这应包括被搜索的语料库、\fBrg\fP 命令、实际输出和预期输出。还请包括使用 \fB\-\-debug\fP 标志运行相同 \fBrg\fP 命令的输出。
.sp
如果您的问题不能明确归入"bug"或"feature request"类别，那么欢迎您将其发布到问题跟踪器的 Discussions 部分：\fIhttps://github.com/BurntSushi/ripgrep/discussions\fP。
.
.
.SH 作者
Andrew Gallant <\fIjamslam@gmail.com\fP>

flag-hyperlink-format-long-prefix = 设置打印结果时使用的超链接格式。超链接使 ripgrep 输出的某些元素（如文件路径）可以点击。这通常只在支持 OSC-8 超链接的终端模拟器中有效。例如，格式 \fBfile://{host}{path}\fP 将发出 RFC 8089 超链接。要查看 ripgrep 正在使用的格式，请传递 \flag{debug} 标志。
.sp
或者，格式字符串也可以对应于以下别名之一：

flag-hyperlink-format-long-suffix = 别名将被替换为旨在用于相应应用程序的格式字符串。
.sp
格式字符串中可以使用以下变量：
.sp
.TP 12
\fB{path}\fP
必需。这会被替换为匹配文件的路径。该路径保证是绝对的，并且经过百分号编码，因此可以安全地放入 URI。请注意，路径保证以 / 开头。
.TP 12
\fB{host}\fP
可选。这会被替换为您系统的主机名。在 Unix 上，这对应于调用 \fBgethostname\fP。在 Windows 上，这对应于调用 \fBGetComputerNameExW\fP 以获取系统的"物理 DNS 主机名"。或者，如果提供了 \flag{hostname-bin}，那么将返回该程序输出中的主机名。如果找不到主机名，那么此变量会被替换为空字符串。
.TP 12
\fB{line}\fP
可选。如果合适，这会被替换为匹配的行号。如果没有可用的行号（例如，如果给出了 \fB\-\-no\-line\-number\fP），那么它会自动替换为值 1。
.TP 12
\fB{column}\fP
可选，但要求存在 \fB{line}\fP。如果合适，这会被替换为匹配的列号。如果没有可用的列号（例如，如果给出了 \fB\-\-no\-column\fP），那么它会自动替换为值 1。
.TP 12
\fB{wslprefix}\fP
可选。这是一个特殊值，设置为 \fBwsl$/\fP\fIWSL_DISTRO_NAME\fP，其中 \fIWSL_DISTRO_NAME\fP 对应于等价环境变量的值。如果系统不是 Unix，或者未设置 \fIWSL_DISTRO_NAME\fP 环境变量，那么这会被替换为空字符串。
.PP
格式字符串可以为空。空格式字符串等同于 \fBnone\fP 别名。在这种情况下，超链接将被禁用。
.sp
目前，ripgrep 默认不启用超链接。用户必须选择使用它们。如果您不确定使用什么格式，请尝试 \fBdefault\fP。
.sp
与颜色一样，当 ripgrep 检测到 stdout 未连接到 tty 时，超链接会自动禁用，无论此标志的值如何。用户可以通过 \fB\-\-color=always\fP 强制发出超链接。
.sp
请注意，只有当输出中同时有路径并且启用了颜色时，才会写入超链接。要在没有颜色的情况下写入超链接，您需要配置 ripgrep 不进行任何着色，而无需完全禁用所有 ANSI 转义码：
.sp
.EX
    \-\-colors 'path:none' \\
    \-\-colors 'line:none' \\
    \-\-colors 'column:none' \\
    \-\-colors 'match:none'
.EE
.sp
ripgrep 之所以这样工作，是因为它将 \flag{color} 标志视为是否应该使用 ANSI 转义码的代理。这意味着像 \fBNO_COLOR=1\fP 和 \fBTERM=dumb\fP 这样的环境变量不仅会禁用颜色，还会禁用超链接。同样，当 ripgrep 不写入 tty 时，颜色和超链接都会被禁用。（除非通过设置 \fB\-\-color=always\fP 强制解决这个问题。）
.sp
如果您直接搜索文件，例如：
.sp
.EX
    rg foo path/to/file
.EE
.sp
那么不会发出超链接，因为给定的路径不会出现在输出中。要使路径出现（从而也出现超链接），请使用 \flag{with-filename} 标志。
.sp
有关终端模拟器中超链接的更多信息，请参阅：
https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda

msg-prefix = rg

indexing-not-supported = 此版本的 ripgrep 不支持索引。

err-choice-unrecognized = 选项 '{choice}' 无法识别

err-config-line = {line}: {err}

err-config-open = {path}: {err}

err-config-read-path = 无法读取 RIPGREP_CONFIG_PATH 指定的文件：{err}

err-flag-no-indexing = --{flag} 标志不支持索引

err-flag-repeated-3-times = 该标志最多只能重复 3 次

err-get-cwd = 获取当前工作目录失败：{err}
您的 CWD 是否已被删除？

err-index-at-most-twice = -X/--index 最多只能给出两次

err-index-create = 找不到创建/更新索引的位置

err-index-read = 找不到要读取的索引

err-indexing-disabled = 此版本的 ripgrep 未启用索引

err-init-logger = 初始化日志器失败：{err}

err-invalid-hyperlink-format = 无效的超链接格式

err-invalid-size = 无效的大小

err-missing-value = 标志 {flag} 缺少值

err-need-pattern = ripgrep 需要至少一个模式才能执行搜索

err-nothing-searched = 没有搜索任何文件，这可能意味着 ripgrep 应用了您未预期的过滤器。
使用 --debug 运行将显示文件被跳过的原因。

err-parsing-flag = 解析标志 {flag} 时出错

err-path-separator = 路径分隔符必须恰好为一个字节，但给定的分隔符为 {len} 字节：{sep}
在 Windows 的某些 shell 中，'/' 会被自动展开。请改用 '//'。

err-pattern-not-utf8 = 给定的模式不是有效的 UTF-8

err-pcre2-unavailable = 此版本的 ripgrep 不提供 PCRE2

err-preprocessor-failed = 预处理命令执行失败：'{cmd}'：{err}

err-preprocessor-start = 预处理命令无法启动：'{cmd}'：{err}

err-regex-engine-unrecognized = 无法识别的正则引擎 '{engine}'

err-regex-hybrid = 使用默认正则引擎和 PCRE2 均无法编译该正则表达式。

默认正则引擎错误：
{divider}
{rust_err}
{divider}

PCRE2 正则引擎错误：
{pcre_err}

err-separator-utf8 = 分隔符必须是有效的 UTF-8（可使用转义序列提供非有效 UTF-8 的分隔符）

err-similar-flags = 可用的相似标志：{list}

err-size-too-big = 大小过大

err-sort-created = 不支持按创建时间排序：{err}

err-sort-last-accessed = 不支持按最后访问时间排序：{err}

err-sort-last-modified = 不支持按最后修改时间排序：{err}

err-stdin-consumed = 从标准输入读取 -f/--file 出错：标准输入已被使用

err-stdin-pattern-and-search = 错误：在搜索标准输入的同时尝试从标准输入读取模式

err-unrecognized-flag-long = 无法识别的标志 --{name}

err-unrecognized-flag-short = 无法识别的标志 -{name}

err-value-not-number = 值不是有效的数字

err-value-not-utf8 = 值不是有效的 UTF-8

help-flag-negated = 可以使用 --{flag} 禁用此标志。

man-flag-negated = 可以使用 \fB\-\-{flag}\fP 禁用此标志。

log-binary-detection = {path}: 二进制检测：{bin}

log-config-env-unset = 未设置 RIPGREP_CONFIG_PATH 环境变量，因此不读取任何配置文件

log-cwd-env = 从环境中读取 CWD：{cwd}

log-haystack-ignored = 忽略 {path}: 未通过 haystack 过滤器：文件类型：{file_type}，元数据：{metadata}

log-heuristic-cwd = 启发式规则选择搜索 ./

log-heuristic-stdin = 启发式规则选择搜索标准输入

log-hostname-empty = 命令 '{bin}' 在去除首尾空白后的输出为空（回退到平台主机名）

log-hostname-failed = 无法获取主机名：{err}

log-hostname-found = 找到用于超链接配置的主机名：{hostname}

log-hostname-not-utf8 = 获取的主机名 {hostname} 不是有效的 UTF-8

log-hostname-read-failed = 无法读取命令 '{bin}' 的输出以获取主机名（回退到平台主机名）：{err}

log-hostname-run-failed = 无法运行命令 '{bin}' 以获取主机名（回退到平台主机名）：{err}

log-hostname-spawn-failed = 无法启动命令 '{bin}' 以获取主机名（回退到平台主机名）：{err}

log-hybrid-rust-error = 在混合模式下构建 Rust 正则表达式时出错：
{err}

log-hyperlink-format = 超链接格式：{fmt}

log-is-one-file = is_one_file? {is_one_file}

log-no-config = 由于存在 --no-config，不读取配置文件

log-no-config-args = 未从配置文件中找到额外参数

log-paths-count = 给定搜索的路径数量：{count}

log-stdin-heuristic = 使用启发式规则确定是从标准输入读取还是搜索 ./（is_readable_stdin={is_readable_stdin}，stdin_consumed={stdin_consumed}，mode={mode}）

log-using-threads = 使用 {threads} 个线程

log-wsl-not-utf8 = 找到 WSL_DISTRO_NAME={distro}，但该值不是 UTF-8

log-wsl-prefix-found = 找到用于超链接配置的 wsl_prefix：{wsl_prefix}

stats-summary = {matches} 个匹配
{lines} 个匹配行
{searches_with_match} 个文件包含匹配
{searches} 个文件被搜索
{bytes_printed} 字节已打印
{bytes_searched} 字节已搜索
{search_time} 秒用于搜索
{process_time} 秒总计

suggest-multiline = 请考虑使用 --multiline 标志（或简写 -U）启用多行模式。
启用多行模式后，可以匹配换行符。

suggest-pcre2 = 请考虑使用 --pcre2 标志启用 PCRE2，它可以处理反向引用和环视。

suggest-text = 请考虑使用 --text 标志（或简写 -a）启用文本模式。否则，二进制检测会开启，且无法匹配 NUL 字节。

version-features = 特性：{features}

version-pcre2-available = PCRE2 {major}.{minor} 可用

version-pcre2-jit-available = （JIT 可用）

version-pcre2-jit-unavailable = （JIT 不可用）

version-pcre2-unavailable = 此版本的 ripgrep 不提供 PCRE2。

version-rev = {version}（修订 {hash}）

version-short = ripgrep {digits}

version-simd-compile = simd(compile):{features}

version-simd-runtime = simd(runtime):{features}
