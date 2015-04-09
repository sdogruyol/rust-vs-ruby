require 'cuba'
require 'json'

# To launch just type: 'rackup' in your console
Cuba.define do
  on get do

    on root do
      data = { first_name: 'Serdar', last_name: 'Dogruyol' }
      res.headers["Content-Type"] = "application/json; charset=utf-8"
      res.write data.to_json
    end

  end
end
