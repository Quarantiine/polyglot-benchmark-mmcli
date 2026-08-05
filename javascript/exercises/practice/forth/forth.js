//
// This is only a SKELETON file for the 'Forth' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export class Forth {
  constructor() {
    this._stack = [];
    this.dictionary = new Map();

    // Register built-in words
    this.dictionary.set('+', (stack) => {
      if (stack.length < 2) throw new Error('Stack empty');
      const b = stack.pop();
      const a = stack.pop();
      stack.push(a + b);
    });

    this.dictionary.set('-', (stack) => {
      if (stack.length < 2) throw new Error('Stack empty');
      const b = stack.pop();
      const a = stack.pop();
      stack.push(a - b);
    });

    this.dictionary.set('*', (stack) => {
      if (stack.length < 2) throw new Error('Stack empty');
      const b = stack.pop();
      const a = stack.pop();
      stack.push(a * b);
    });

    this.dictionary.set('/', (stack) => {
      if (stack.length < 2) throw new Error('Stack empty');
      const b = stack.pop();
      const a = stack.pop();
      if (b === 0) throw new Error('Division by zero');
      stack.push(Math.trunc(a / b));
    });

    this.dictionary.set('dup', (stack) => {
      if (stack.length < 1) throw new Error('Stack empty');
      stack.push(stack[stack.length - 1]);
    });

    this.dictionary.set('drop', (stack) => {
      if (stack.length < 1) throw new Error('Stack empty');
      stack.pop();
    });

    this.dictionary.set('swap', (stack) => {
      if (stack.length < 2) throw new Error('Stack empty');
      const b = stack.pop();
      const a = stack.pop();
      stack.push(b);
      stack.push(a);
    });

    this.dictionary.set('over', (stack) => {
      if (stack.length < 2) throw new Error('Stack empty');
      stack.push(stack[stack.length - 2]);
    });
  }

  evaluate(text) {
    const tokens = this._tokenize(text);
    let i = 0;
    while (i < tokens.length) {
      const token = tokens[i];
      if (token === ':') {
        i++;
        if (i >= tokens.length) throw new Error('Invalid definition');
        const wordName = tokens[i].toLowerCase();
        if (!isNaN(Number(wordName))) {
          throw new Error('Invalid definition');
        }
        i++;
        const body = [];
        let closed = false;
        while (i < tokens.length) {
          const t = tokens[i];
          if (t === ';') {
            closed = true;
            i++;
            break;
          } else {
            body.push(t);
            i++;
          }
        }
        if (!closed) throw new Error('Invalid definition');

        // Compile body: resolve existing dictionary words at definition time
        const compiledBody = this._compileBody(body);
        this.dictionary.set(wordName, compiledBody);
      } else {
        this._executeToken(token, this._stack);
        i++;
      }
    }
  }

  _tokenize(text) {
    return text.trim().split(/\s+/).filter(Boolean);
  }

  _compileBody(bodyTokens) {
    const result = [];
    for (const t of bodyTokens) {
      const lowerT = t.toLowerCase();
      if (!isNaN(Number(t))) {
        result.push(t);
      } else if (this.dictionary.has(lowerT)) {
        const entry = this.dictionary.get(lowerT);
        if (Array.isArray(entry)) {
          result.push(...entry);
        } else {
          result.push(lowerT);
        }
      } else {
        result.push(lowerT);
      }
    }
    return result;
  }

  _executeTokens(tokens, stack) {
    for (const t of tokens) {
      this._executeToken(t, stack);
    }
  }

  _executeToken(token, stack) {
    const lower = token.toLowerCase();
    if (!isNaN(Number(token))) {
      stack.push(Number(token));
    } else if (this.dictionary.has(lower)) {
      const entry = this.dictionary.get(lower);
      if (typeof entry === 'function') {
        entry(stack);
      } else if (Array.isArray(entry)) {
        this._executeTokens(entry, stack);
      }
    } else {
      throw new Error('Unknown command');
    }
  }

  get stack() {
    return [...this._stack];
  }
}
