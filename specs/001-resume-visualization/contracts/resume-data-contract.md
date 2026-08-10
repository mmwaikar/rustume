# Contract: Resume Input and Visualization Payloads

## Input contract: JSON Resume document

The application will consume a JSON object that conforms to the JSON Resume schema. At minimum, the file should provide:

```json
{
  "basics": {
    "name": "Jane Doe",
    "label": "Software Engineer",
    "location": {
      "countryCode": "US"
    }
  },
  "work": [
    {
      "name": "Example Corp",
      "position": "Senior Engineer",
      "startDate": "2020-01",
      "endDate": "2024-06",
      "summary": "Led platform improvements.",
      "highlights": ["Improved reliability"]
    }
  ],
  "skills": [
    {
      "name": "Rust",
      "level": "Advanced"
    }
  ]
}
```

## Output contract: visualization payload

The app will normalize the resume into two internally consumed payloads:

1. Experience chart payload
   - `company`, `position`, `duration_months`, `country`
2. Graph payload
   - `nodes` with `id`, `label`, `group`, `size`
   - `edges` with `source`, `target`, `label`, `weight`

These payloads are the contract between the Rust data-shaping layer and the front-end visualization components.
