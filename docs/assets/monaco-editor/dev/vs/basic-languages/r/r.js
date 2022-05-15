/*!-----------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Version: 0.32.1(29a273516805a852aa8edc5e05059f119b13eff0)
 * Released under the MIT license
 * https://github.com/microsoft/monaco-editor/blob/main/LICENSE.txt
 *-----------------------------------------------------------------------------*/
define("vs/basic-languages/r/r", ["require"],(require)=>{
var moduleExports = (() => {
  var __defProp = Object.defineProperty;
  var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
  var __getOwnPropNames = Object.getOwnPropertyNames;
  var __hasOwnProp = Object.prototype.hasOwnProperty;
  var __markAsModule = (target) => __defProp(target, "__esModule", { value: true });
  var __export = (target, all) => {
    for (var name in all)
      __defProp(target, name, { get: all[name], enumerable: true });
  };
  var __reExport = (target, module, copyDefault, desc) => {
    if (module && typeof module === "object" || typeof module === "function") {
      for (let key of __getOwnPropNames(module))
        if (!__hasOwnProp.call(target, key) && (copyDefault || key !== "default"))
          __defProp(target, key, { get: () => module[key], enumerable: !(desc = __getOwnPropDesc(module, key)) || desc.enumerable });
    }
    return target;
  };
  var __toCommonJS = /* @__PURE__ */ ((cache) => {
    return (module, temp) => {
      return cache && cache.get(module) || (temp = __reExport(__markAsModule({}), module, 1), cache && cache.set(module, temp), temp);
    };
  })(typeof WeakMap !== "undefined" ? /* @__PURE__ */ new WeakMap() : 0);

  // src/basic-languages/r/r.ts
  var r_exports = {};
  __export(r_exports, {
    conf: () => conf,
    language: () => language
  });
  var conf = {
    comments: {
      lineComment: "#"
    },
    brackets: [
      ["{", "}"],
      ["[", "]"],
      ["(", ")"]
    ],
    autoClosingPairs: [
      { open: "{", close: "}" },
      { open: "[", close: "]" },
      { open: "(", close: ")" },
      { open: '"', close: '"' }
    ],
    surroundingPairs: [
      { open: "{", close: "}" },
      { open: "[", close: "]" },
      { open: "(", close: ")" },
      { open: '"', close: '"' }
    ]
  };
  var language = {
    defaultToken: "",
    tokenPostfix: ".r",
    roxygen: [
      "@alias",
      "@aliases",
      "@assignee",
      "@author",
      "@backref",
      "@callGraph",
      "@callGraphDepth",
      "@callGraphPrimitives",
      "@concept",
      "@describeIn",
      "@description",
      "@details",
      "@docType",
      "@encoding",
      "@evalNamespace",
      "@evalRd",
      "@example",
      "@examples",
      "@export",
      "@exportClass",
      "@exportMethod",
      "@exportPattern",
      "@family",
      "@field",
      "@formals",
      "@format",
      "@import",
      "@importClassesFrom",
      "@importFrom",
      "@importMethodsFrom",
      "@include",
      "@inherit",
      "@inheritDotParams",
      "@inheritParams",
      "@inheritSection",
      "@keywords",
      "@md",
      "@method",
      "@name",
      "@noMd",
      "@noRd",
      "@note",
      "@param",
      "@rawNamespace",
      "@rawRd",
      "@rdname",
      "@references",
      "@return",
      "@S3method",
      "@section",
      "@seealso",
      "@setClass",
      "@slot",
      "@source",
      "@template",
      "@templateVar",
      "@title",
      "@TODO",
      "@usage",
      "@useDynLib"
    ],
    constants: [
      "NULL",
      "FALSE",
      "TRUE",
      "NA",
      "Inf",
      "NaN",
      "NA_integer_",
      "NA_real_",
      "NA_complex_",
      "NA_character_",
      "T",
      "F",
      "LETTERS",
      "letters",
      "month.abb",
      "month.name",
      "pi",
      "R.version.string"
    ],
    keywords: [
      "break",
      "next",
      "return",
      "if",
      "else",
      "for",
      "in",
      "repeat",
      "while",
      "array",
      "category",
      "character",
      "complex",
      "double",
      "function",
      "integer",
      "list",
      "logical",
      "matrix",
      "numeric",
      "vector",
      "data.frame",
      "factor",
      "library",
      "require",
      "attach",
      "detach",
      "source"
    ],
    special: ["\\n", "\\r", "\\t", "\\b", "\\a", "\\f", "\\v", "\\'", '\\"', "\\\\"],
    brackets: [
      { open: "{", close: "}", token: "delimiter.curly" },
      { open: "[", close: "]", token: "delimiter.bracket" },
      { open: "(", close: ")", token: "delimiter.parenthesis" }
    ],
    tokenizer: {
      root: [
        { include: "@numbers" },
        { include: "@strings" },
        [/[{}\[\]()]/, "@brackets"],
        { include: "@operators" },
        [/#'$/, "comment.doc"],
        [/#'/, "comment.doc", "@roxygen"],
        [/(^#.*$)/, "comment"],
        [/\s+/, "white"],
        [/[,:;]/, "delimiter"],
        [/@[a-zA-Z]\w*/, "tag"],
        [
          /[a-zA-Z]\w*/,
          {
            cases: {
              "@keywords": "keyword",
              "@constants": "constant",
              "@default": "identifier"
            }
          }
        ]
      ],
      roxygen: [
        [
          /@\w+/,
          {
            cases: {
              "@roxygen": "tag",
              "@eos": { token: "comment.doc", next: "@pop" },
              "@default": "comment.doc"
            }
          }
        ],
        [
          /\s+/,
          {
            cases: {
              "@eos": { token: "comment.doc", next: "@pop" },
              "@default": "comment.doc"
            }
          }
        ],
        [/.*/, { token: "comment.doc", next: "@pop" }]
      ],
      numbers: [
        [/0[xX][0-9a-fA-F]+/, "number.hex"],
        [/-?(\d*\.)?\d+([eE][+\-]?\d+)?/, "number"]
      ],
      operators: [
        [/<{1,2}-/, "operator"],
        [/->{1,2}/, "operator"],
        [/%[^%\s]+%/, "operator"],
        [/\*\*/, "operator"],
        [/%%/, "operator"],
        [/&&/, "operator"],
        [/\|\|/, "operator"],
        [/<</, "operator"],
        [/>>/, "operator"],
        [/[-+=&|!<>^~*/:$]/, "operator"]
      ],
      strings: [
        [/'/, "string.escape", "@stringBody"],
        [/"/, "string.escape", "@dblStringBody"]
      ],
      stringBody: [
        [
          /\\./,
          {
            cases: {
              "@special": "string",
              "@default": "error-token"
            }
          }
        ],
        [/'/, "string.escape", "@popall"],
        [/./, "string"]
      ],
      dblStringBody: [
        [
          /\\./,
          {
            cases: {
              "@special": "string",
              "@default": "error-token"
            }
          }
        ],
        [/"/, "string.escape", "@popall"],
        [/./, "string"]
      ]
    }
  };
  return __toCommonJS(r_exports);
})();
return moduleExports;
});
