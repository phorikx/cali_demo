defmodule FrontendWeb.PageController do
  use FrontendWeb, :controller

  def home(conn, _params) do
    {:ok, channel} = GRPC.Stub.connect("localhost:50570")
    request = NumberService.NumberRequest.new(name: "Paul")
    {:ok, response} = NumberService.Stub.get_number(channel, request)
    IO.inspect(response)
    
    %{number: number} = response
    
    render(conn, :home, layout: false, number: number)
  end
end
