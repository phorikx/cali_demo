defmodule NumberService.Service do
  use GRPC.Service, name: "number_service.NumberService"

  rpc :GetNumber, NumberService.NumberRequest, NumberService.NumberReply
end

defmodule NumberService.Stub do
  use GRPC.Stub, service: NumberService.Service
end
