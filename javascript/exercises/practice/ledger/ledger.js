class LedgerEntry {
  constructor() {
    this.date = undefined;
    this.description = undefined;
    this.change = undefined;
  }
}

export function createEntry(date, description, change) {
  let entry = new LedgerEntry();
  // Parse date string assuming UTC or local correctly to avoid timezone shifts
  // Given YYYY-MM-DD, new Date(date + 'T00:00:00') ensures consistent interpretation
  entry.date = new Date(`${date}T00:00:00`);
  entry.description = description;
  entry.change = change;
  return entry;
}

export function formatEntries(currency, locale, entries) {
  if (!['en-US', 'nl-NL'].includes(locale)) {
    throw new Error('Invalid locale');
  }
  if (!['USD', 'EUR'].includes(currency)) {
    throw new Error('Invalid currency');
  }

  // Clone entries to avoid mutating the original array passed in tests
  const sortedEntries = [...entries].sort((a, b) => {
    if (a.date.getTime() !== b.date.getTime()) {
      return a.date.getTime() - b.date.getTime();
    }
    if (a.change !== b.change) {
      return a.change - b.change;
    }
    return a.description.localeCompare(b.description);
  });

  let table = '';

  if (locale === 'en-US') {
    table +=
      'Date'.padEnd(10, ' ') +
      ' | ' +
      'Description'.padEnd(25, ' ') +
      ' | ' +
      'Change'.padEnd(13, ' ');

    sortedEntries.forEach((entry) => {
      table += '\n';
      const month = (entry.date.getMonth() + 1).toString().padStart(2, '0');
      const day = entry.date.getDate().toString().padStart(2, '0');
      const year = entry.date.getFullYear();
      const dateStr = `${month}/${day}/${year}`;
      table += `${dateStr} | `;

      const truncatedDescription =
        entry.description.length > 25
          ? `${entry.description.substring(0, 22)}...`
          : entry.description.padEnd(25, ' ');
      table += `${truncatedDescription} | `;

      const amount = Math.abs(entry.change) / 100;
      const formattedNum = amount.toLocaleString('en-US', {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
      });

      let symbol = '$';
      if (currency === 'EUR') {
        symbol = '€';
      }

      let changeStr = '';
      if (entry.change < 0) {
        changeStr = `(${symbol}${formattedNum})`;
      } else {
        changeStr = `${symbol}${formattedNum} `;
      }
      table += changeStr.padStart(13, ' ');
    });
  } else if (locale === 'nl-NL') {
    table +=
      'Datum'.padEnd(10, ' ') +
      ' | ' +
      'Omschrijving'.padEnd(25, ' ') +
      ' | ' +
      'Verandering'.padEnd(13, ' ');

    sortedEntries.forEach((entry) => {
      table += '\n';
      const day = entry.date.getDate().toString().padStart(2, '0');
      const month = (entry.date.getMonth() + 1).toString().padStart(2, '0');
      const year = entry.date.getFullYear();
      const dateStr = `${day}-${month}-${year}`;
      table += `${dateStr} | `;

      const truncatedDescription =
        entry.description.length > 25
          ? `${entry.description.substring(0, 22)}...`
          : entry.description.padEnd(25, ' ');
      table += `${truncatedDescription} | `;

      const amount = entry.change / 100;
      const formattedNum = Math.abs(amount).toLocaleString('nl-NL', {
        minimumFractionDigits: 2,
        maximumFractionDigits: 2,
      });

      let symbol = '$';
      if (currency === 'EUR') {
        symbol = '€';
      }

      let changeStr = '';
      if (entry.change < 0) {
        changeStr = `${symbol}\xa0${amount.toLocaleString('nl-NL', {
          minimumFractionDigits: 2,
          maximumFractionDigits: 2,
        })} `;
      } else {
        changeStr = `${symbol}\xa0${formattedNum} `;
      }
      table += changeStr.padStart(13, ' ');
    });
  }

  return table;
}
