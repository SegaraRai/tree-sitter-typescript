from unittest import TestCase

import tree_sitter, tree_sitter_typescript
from tree_sitter import Language, Parser


class TestLanguage(TestCase):
    def test_can_load_typescript_grammar(self):
        try:
            Parser(Language(tree_sitter_typescript.language_typescript()))
        except Exception:
            self.fail("Error loading TypeScript grammar")

    def test_can_load_tsx_grammar(self):
        try:
            Parser(Language(tree_sitter_typescript.language_tsx()))
        except Exception:
            self.fail("Error loading TSX grammar")
