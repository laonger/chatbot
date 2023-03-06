curl https://api.openai.com/v1/chat/completions \
  -H 'Content-Type: application/json' \
  -H "Authorization: Bearer $OPANAI_API_KEY" \
  -d '{
  "model": "gpt-3.5-turbo",
  "messages": [{"role": "user", "content": "Hello!"}]
}'

