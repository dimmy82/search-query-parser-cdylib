# what is this library for

this is a cdylib library based on [search-query-parser](https://github.com/dimmy82/search-query-parser), it can be 
called from JVM language via JNI.

# usage

## Java

### 1. prepare the cdylib for your java project

1. you can download the cdylib binary from [bin/](https://github.com/dimmy82/search-query-parser-cdylib/tree/master/bin) or you can run `cargo build --release`, the cdylib binary will be output to target/release

2. move the cdylib binary to any path you like 

### 2. create java class to link the cdylib binary

```javascript
class SearchQueryParser {
    // This declares that the static `hello` method will be provided
    // a native library.
    public static native String parseQueryToCondition(String queryString);

    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.loadLibrary("search_query_parser_0.1.4");
    }
}
```

#### Notice

1. DO NOT change the class name `SearchQueryParser` and the static method `parseQueryToCondition`

2. the library name `search_query_parser_0.1.4` should be same to the cdylib name `libsearch_query_parser_0.1.4.dylib` except the `lib` prefix and `.dylib` extension.

### 3. run java project

```javascript
public class Main {
    public static void main(String[] args) {
        System.out.println(SearchQueryParser.parseQueryToCondition("aaa and (-bbb or \"ccc\")"));
    }
}
```

run
```shell
java -Djava.library.path=${the_full_path_of_the_folder_that_you_move_the_cdylib_binary_to} Main
```

### 4. result

the value that return from the cdylib binary is json string. it looks like
```json
{"and":[{"keyword":"aaa"},{"or":[{"not":{"keyword":"bbb"}},{"phraseKeyword":"ccc"}]}]}
```

## Kotlin

step 1. and 4. are same to the java sample

### 2. create static method to link the cdylib binary

```javascript
@file:JvmName("SearchQueryParser")

external fun parseQueryToCondition(str: String): String
```

### 3. run kotlin project

```javascript
fun main(args: Array<String>) {
    System.loadLibrary("search_query_parser_0.1.4")
    println(parseQueryToCondition("aaa and (-bbb or \"ccc\")"));
}
```

#### Notice

don't forget the `-Djava.library.path`